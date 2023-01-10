use std::net::IpAddr;

use serde::{ Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageType {
    Message(Message),
    Ack(u8),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub sender: (IpAddr, u16),
    pub receiver: IpAddr,
}
