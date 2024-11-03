use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::{App, CurrentScreen, StartUpCachingServer};


pub fn ui(frame: &mut Frame, _app: &App){
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1)
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Caching proxy server",
        Style::default().fg(Color::Blue),
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let mode_layout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
    .split(chunks[1]);

  
    let port = Span::styled(
        _app.port_input.clone(), 
        Style::default().fg(Color::DarkGray)
    );

 
   let origin = Span::styled(
    _app.origin_input.clone(),
        Style::default().fg(Color::DarkGray)
    );

    let input_port =  Paragraph::new(Line::from(port)).block(Block::default().borders(Borders::ALL));
    let input_origin = Paragraph::new(Line::from(origin)).block(Block::default().borders(Borders::ALL));


    frame.render_widget(input_port, mode_layout[0]);
    frame.render_widget(input_origin, mode_layout[1]);

    let pg_request = Paragraph::new(Text::styled(
        _app.log_data.clone().unwrap_or("".to_string()), 
        Style::default().fg(Color::Green)
    )).block(Block::default().borders(Borders::ALL));
    
    frame.render_widget(pg_request, chunks[2]);
}
