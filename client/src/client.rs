use std::net::IpAddr;

use serde::{ Serialize, Deserialize};

/// Client in a chat app
/// name: username, identificator for every username
/// addr: ip address of the user
/// port: port of the client app on the user machine
#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub name: String,
    pub addr: IpAddr,
    pub port: u16,
}
