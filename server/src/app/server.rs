use std::io::Read;
use std::net::TcpListener;
use std::sync::{RwLock, Arc};

use crate::{ error::*, types::* };

/// contains server methods for handling clients and broadcasting messages
pub struct Server;

impl Server {
    /// listenes for new clients
    ///
    /// contains infinite loop
    pub fn listen_for_clients(listener: TcpListener, clients: Arc<RwLock<Vec<Client>>>) -> Result<()>{

        loop {
            let (mut stream, _) = listener.accept()?;
            let mut buf = [0u8; 17];

            let mut bytes_read;
            loop {
                bytes_read = stream.read(&mut buf)?;
                    
                if bytes_read != 0 {
                    break;
                };
            };


            let header = buf[0];
            let MessageHeader::LOG = header.try_into()? else {
                return Err(ServerError::Generic);
            };

            let username = std::str::from_utf8(&buf[1..bytes_read])?;

            let client = Client::new(username.to_string(), stream);

            {
                let mut clients_locked = clients.write()?;
                clients_locked.push(client);
            }
        }
    }

    /// describes the thread receiving part of server
    pub fn recv_thread() {
        
    }

    /// describes the thread sending part of server
    pub fn sending_thread() {
        
    }
}
