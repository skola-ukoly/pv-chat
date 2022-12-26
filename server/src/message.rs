use std::net::IpAddr;

use serde::{Deserialize, Serialize};

use crate::client::Client;

#[derive(Debug, Serialize, Deserialize)]
pub enum MessageType {
    Message(Message),
    Ack(u8),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub sender: Client,
    pub receiver: Client,
    pub timestamp: String,
    pub sequence_number: u8,
    pub body: String,
}
