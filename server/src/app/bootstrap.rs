use crate::{
    error::*,
    config::ServerConfig,
};

pub struct App;

impl App {
    /// Bootstraps the server and its threads
    pub fn run(&self) -> Result<()> {
        let config = ServerConfig::load()?;

        Ok(())
    }
}