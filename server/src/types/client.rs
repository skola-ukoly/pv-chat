use std::net::UdpSocket;

pub struct Client {
    pub name: String,
    pub socket: UdpSocket,
}