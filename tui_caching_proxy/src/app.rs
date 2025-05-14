//! Terminal User Interface (TUI) for the Caching Proxy Server
//! 
//! This module provides a text-based user interface for controlling and
//! monitoring the caching proxy server. It allows users to:
//! 
//! - Configure server port and origin URL
//! - Start/stop the server
//! - View server logs and status
//! 
//! The interface is built using a state-based approach with different screens
//! for different functionality.

use caching_proxy_sever::CachingProxyServer;

/// Represents the current screen being displayed in the TUI
#[derive(Debug)]
pub enum CurrentScreen {
    /// Main screen showing server status and logs
    Main,
    /// Screen for editing server configuration
    Editing,
}

/// Represents which server configuration parameter is being edited
#[derive(Debug)]
pub enum StartUpCachingServer {
    /// Editing the server port
    Port,
    /// Editing the origin server URL
    Origin,
}

/// Main application state for the TUI
#[derive(Debug)]
pub struct App {
    /// Input field for the server port
    pub port_input: String,              
    /// Input field for the origin server URL
    pub origin_input: String,            
    /// Current screen being displayed
    pub current_screen: CurrentScreen,
    /// Server log messages
    pub log_data: Option<String>, 
    /// Currently active configuration parameter being edited
    pub request_information: Option<StartUpCachingServer>,
}
 
impl App {
    /// Creates a new instance of the TUI application
    /// 
    /// # Returns
    /// 
    /// Returns a new `App` instance with default values
    pub fn new() -> App {
        App {
            port_input: String::new(),
            origin_input: String::new(),
            log_data: None,
            current_screen: CurrentScreen::Main,
            request_information: None,
        }
    }

    /// Starts the caching proxy server with the current configuration
    /// 
    /// This method:
    /// - Creates a new server instance with the configured origin
    /// - Starts the server on the configured port
    /// - Updates the log with startup information
    /// - Resets the input fields
    /// 
    /// # Errors
    /// 
    /// This method will panic if:
    /// - The port number is invalid
    /// - The server fails to start
    pub async fn startup_server(&mut self) {
        let server = CachingProxyServer::new(self.origin_input.clone());
        let _ = server.run(self.port_input.parse().unwrap()).await;
        self.log_data = Some(
            format!("Caching Proxy Server Start app on port {}. Proxy service: {}", self.port_input, self.origin_input)
        );
        self.request_information = None;
        self.port_input = String::new();
        self.origin_input = String::new();
        
    }

}