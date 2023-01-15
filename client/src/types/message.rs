use std::net::IpAddr;

pub enum MessageType {
    Message(Message),
    Ack(u8),
}

pub struct Message {
    pub sender: (IpAddr, u16),
    pub receiver: IpAddr,
}
