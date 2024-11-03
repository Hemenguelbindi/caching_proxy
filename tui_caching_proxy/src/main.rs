use std::{error::Error, io};

use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    Terminal,
};
use tokio::sync::mpsc;

mod app;
mod ui;
use crate::{
    app::{App, CurrentScreen, StartUpCachingServer},
    ui::ui,
};



#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stderr = io::stderr(); // This is a special case. Normally using stdout is fine
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let mut app = App::new();
    let _res = run_app(&mut terminal, &mut app).await;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

async fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }
            match app.current_screen {
                CurrentScreen::Main => match key.code {
                    KeyCode::Char('e') => {
                        app.current_screen = CurrentScreen::Editing;
                        app.request_information = Some(StartUpCachingServer::Port);
                    }
                    KeyCode::Char('q') => {
                        return Ok(true);
                    }
                    _ => {}
                },
                CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
                    match key.code {
                        KeyCode::Enter => {
                            if let Some(editing) = &app.request_information {
                                match editing {
                                    StartUpCachingServer::Port => {
                                        app.request_information = Some(StartUpCachingServer::Origin);
                                    }
                                    StartUpCachingServer::Origin => {
                                        app.startup_server().await;
                                        app.current_screen = CurrentScreen::Main;
                                    }
                                }
                            }
                        }
                        KeyCode::Esc => {
                            app.current_screen = CurrentScreen::Main;
                            app.request_information= None;
                        }
                        KeyCode::Tab => {
                            match app.request_information {
                                Some(StartUpCachingServer::Port) => {
                                    app.request_information = Some(StartUpCachingServer::Origin);
                                }
                                Some(StartUpCachingServer::Origin) => {
                                    app.request_information = Some(StartUpCachingServer::Port);
                                }
                                None => {
                                    app.request_information = None;
                                }
                            }
                        }
                        KeyCode::Backspace => {
                            if let Some(editing) = &app.request_information {
                                match editing {
                                    StartUpCachingServer::Port => {
                                        app.port_input.pop();
                                    }
                                    StartUpCachingServer::Origin => {
                                        app.origin_input.pop();
                                    }
                                }
                            }
                        }
                        KeyCode::Char(value) => {
                            if let Some(editing) = &app.request_information {
                                match editing {
                                    StartUpCachingServer::Port => {
                                        app.port_input.push(value);
                                    }
                                    StartUpCachingServer::Origin => {
                                        app.origin_input.push(value);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
    }
}