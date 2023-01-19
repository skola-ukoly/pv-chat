use crate::error::*;

use cipher::Cipher;


pub struct Controller {
    cipher: Cipher,
    net: CustomNet,
}

impl Controller {
    pub fn new(cipher: Cipher, net: CustomNet) -> Self {
        Self {
            cipher
            net
        } 
    }
    pub fn connect() {}

    pub fn disconnect() {}

    pub fn send_message() {}

    pub fn update() {}
}
