use clap::Parser;

/// arguments for chat client 
#[derive(Parser, Debug)]
pub struct ClientArgs {
    /// ip address of a chat server
    pub server: String,
    /// port of the chat server
    pub port: u16,
    /// your username in the chat
    pub name: String,
}
