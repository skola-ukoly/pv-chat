use std::{net::TcpListener, sync::{RwLock, Arc, mpsc, Mutex}};

use crate::{
    error::*,
    config::ServerConfig,
    types::*,
    app::server::Server,
};

pub struct App;

impl App {
    /// Bootstraps the server and its threads
    pub fn run(&self) -> Result<()> {
        let config = ServerConfig::load()?;
        let addr_port = format!("{}:{}", config.address, config.port);
        let listener = TcpListener::bind(addr_port)?;

        let clients = Arc::new(Mutex::new(Vec::new()));
        
        let (tx, rx) = mpsc::channel::<Message>();
        
        Server::recv_thread(Arc::clone(&clients), tx)?;
//        Server::sending_thread(Arc::clone(&clients), rx)?;

        
        
        Server::listen_for_clients(listener, Arc::clone(&clients))?;
        Ok(())
    }
}
