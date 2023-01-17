use std::net::TcpStream;

#[derive(Debug)]
pub struct Client {
    pub name: String, 
    pub stream: TcpStream,
}

impl Client {
    pub fn new(name: String, stream: TcpStream) -> Self {
        Self {
            name, 
            stream,
        }
    }
}
