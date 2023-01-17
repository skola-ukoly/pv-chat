use std::{net::TcpListener, sync::{RwLock, Arc}};

use crate::{
    error::*,
    config::ServerConfig,
    app::server::Server,
};

pub struct App;

impl App {
    /// Bootstraps the server and its threads
    pub fn run(&self) -> Result<()> {
        let config = ServerConfig::load()?;
        let addr_port = format!("{}:{}", config.address, config.port);
        let listener = TcpListener::bind(addr_port)?;

        let clients = Arc::new(RwLock::new(Vec::new()));

        Server::listen_for_clients(listener, clients)?;


        Ok(())
    }
}
