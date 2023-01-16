use crate::types::Client;

use std::net::UdpSocket;

struct App {
    socket: UdpSocket,
    clients: Vec<Client>,
}

impl App {
    /// describes the thread receiving part of server
    pub fn recv_thread() {
        
    }

    /// describes the thread sending part of server
    pub fn sending_thread() {
        
    }
}