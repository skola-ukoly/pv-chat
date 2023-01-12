mod error;
mod server_config;
mod client;
mod message;

use crate::{
    error::ChatError,
    server_config::ServerConfig
};

use std::net::UdpSocket;

use config::Config;

#[tokio::main]
async fn main() -> Result<(), ChatError> {
    let config: ServerConfig = Config::builder()
        .add_source(config::File::with_name("Config.toml"))
        .build()
        .expect("config could not be loaded")
        .try_deserialize()
        .expect("configuration was in invalid format (check src/server_config.rs for expected structure)");

    let socket = UdpSocket::bind(format!("{}:{}", config.address, config.port))?;

    loop {
        let mut buf = [0u8; 1024];
        let (amt, src) = socket.recv_from(&mut buf)?;
    }

}
