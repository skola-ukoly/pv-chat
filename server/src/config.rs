use crate::error::Result;

use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerConfig {
    pub address: String,
    pub port: u16,
}


impl ServerConfig {
    pub fn load() -> Result<Config> {
        let config = Config::builder()
            .add_source(config::File::with_name("Config.toml"))
            .build()?;

        Ok(config)
    }
}