use caching_proxy_sever::CachingProxyServer;

pub enum CurrentScreen {
    Main,
    Editing,
}

pub enum StartUpCachingServer {
    Port,
    Origin,
}

pub struct App {
    pub port_input: String,              
    pub origin_input: String,            
    pub current_screen: CurrentScreen,
    pub log_data: Option<String>, 
    pub request_information: Option<StartUpCachingServer>,
}
 
impl App {
    pub fn new() -> App {
        App {
            port_input: String::new(),
            origin_input: String::new(),
            log_data: None,
            current_screen: CurrentScreen::Main,
            request_information: None,
        }
    }

    
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