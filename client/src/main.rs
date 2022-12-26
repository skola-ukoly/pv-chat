mod client_args;
mod error;
mod client;
mod message;
mod connection;

use std::sync::Arc;

use clap::Parser;
use client::Client;
use error::ChatError;
use tokio::net::UdpSocket;

use crate::client_args::ClientArgs;

#[tokio::main]
async fn main() -> Result<(), ChatError> {
    let args: ClientArgs = ClientArgs::parse();

    let socket = Arc::new(UdpSocket::bind(format!("127.0.0.1:{}", args.port)).await?);
    
    Ok(())
}
