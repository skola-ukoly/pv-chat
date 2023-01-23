use std::net::TcpStream;

pub struct CustomNet {
    server_addr: String,
    server_port: u16,
    username: String,
    stream: TcpStream,
}

impl CustomNet {

    pub fn connect(server_addr: String, server_port: u16, username: String) -> Self {
        let stream = TcpStream::connect(format!("{}:{}", server_addr, server_port)).unwrap();
        Self {
            server_addr,
            server_port,
            username,
            stream,
        }
    }
}
