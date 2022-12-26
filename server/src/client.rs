use std::net::IpAddr;

use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    pub name: String,
    pub addr: IpAddr,
    pub port: u16,
}

