use crate::error::*;
use std::str;

#[derive(Debug)]
pub enum MessageHeader {
    LOG = 0,
    DIS = 1,
    MSG = 2,
    ACK = 3,
    FIN = 4,
}

impl TryFrom<u8> for MessageHeader {
    type Error = ServerError;
    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(MessageHeader::LOG),
            1 => Ok(MessageHeader::DIS),
            2 => Ok(MessageHeader::MSG),
            3 => Ok(MessageHeader::ACK),
            4 => Ok(MessageHeader::FIN),
            _ => Err(ServerError::ParseError)
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub header: MessageHeader,
    pub sender: String,
    pub body: Vec<u8>,
}

impl Message {
    pub fn new(header: MessageHeader, sender: String, body: Vec<u8>) -> Self {
        Self {
            header,
            sender,
            body,
        }
    }    

    pub fn bytes_to_msg(buf: &[u8]) -> Result<Self> {
        if buf.len() < 17 {
            return Err(ServerError::Generic);
        };
        let header = buf[0].try_into()?;
        let sender = str::from_utf8(&buf[1..17])?
            .trim_end_matches('\0')
            .to_string();
        let body = buf[17..].to_vec();

        Ok(Self {
            header,
            sender,
            body,
        })
    }
}