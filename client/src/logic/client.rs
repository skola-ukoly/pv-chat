use std::net::IpAddr;

/// Client in a chat app
/// name: username, identificator for every username
/// addr: ip address of the user
/// port: port of the client app on the user machine
#[derive(Debug)]
pub struct Client {
    pub name: String,
    pub addr: IpAddr,
    pub port: u16,
}
