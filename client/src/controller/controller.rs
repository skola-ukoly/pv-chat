use std::fs::File;

use crate::{error::*, services::CustomNet};

use cipher::Cipher;


pub struct Controller {
    cipher: Cipher,
    net: CustomNet,
}

impl Controller {
    pub fn new(one_time_pad: String, username: String, server_addr: String, server_port: u16) -> Result<Self> {
        let cipher = Cipher::new(File::open(one_time_pad)?);
        let net = CustomNet::connect(server_addr, server_port, username);

        Ok(Self {
            cipher,
            net,
        }) 
    }
    pub fn connect() {}

    pub fn disconnect() {}

    pub fn send_message() {}

    pub fn update() {}
}
