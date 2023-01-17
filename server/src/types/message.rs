use crate::error::*;

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

pub struct Message<'a> {
    pub sender: String,
    pub body: &'a [u8],
}

