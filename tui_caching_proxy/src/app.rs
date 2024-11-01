use std::collections::HashMap;


pub enum CurrentScreen {
    Main,
    RunServer,
    Exiting,
}


pub enum RunServer {
    Port,
    Origin
}


pub struct App {
    pub port_input: String,
    pub origin_input: String,
    pub current_screen: CurrentScreen,
    pub run_server: Option<RunServer>,
    pub pairs: HashMap<String, String>
}


impl App {
    pub fn new() -> Self {
        Self
        {
            port_input: String::new(),
            origin_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            run_server: None,
        }
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.port_input.clone(), self.origin_input.clone());

        self.port_input = String::new();
        self.origin_input = String::new();
        self.run_server = None;
    }

    pub fn run_server(&mut self){
        if let Some(server) = &self.run_server {
            match server{
               RunServer::Port => self.run_server = Some(RunServer::Origin),
               RunServer::Origin => self.run_server = Some(RunServer::Port),
            };
        } else {
            self.run_server = Some(RunServer::Port);
        }
    }

    pub fn print_json(&self) -> serde_json::Result<()> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }

}