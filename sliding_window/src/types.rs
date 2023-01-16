use crate::error::*;

/// Datagram header, reserved for sliding window
#[derive(Clone, Copy)]
pub enum DatagramHeader {
    SEND = 0b0000_0000, 
    ACK = 0b0000_0001,
    FIN = 0b0000_0010,
}

impl DatagramHeader {
    fn from(value: u8) -> Result<DatagramHeader> {
        match value {
            0 => Ok(DatagramHeader::SEND),
            1 => Ok(DatagramHeader::ACK),
            2 => Ok(DatagramHeader::FIN),
            _ => Err(SWError::HeaderCouldNotBeParsed)
        }
    }
}

/// Datagram for Sliding Window
pub struct Datagram<'a> {
    pub header: DatagramHeader,
    pub body: &'a [u8]
}

impl<'a> Datagram<'a> {
    /// Encodes a datagram into a bytes buffer
    /// 
    /// # Examples
    /// ```rust
    /// 
    /// use sliding_window::types::*;
    /// 
    /// let dg = Datagram {
    ///     header: DatagramHeader::SEND,
    ///     body: b"Hello world",
    /// };
    /// 
    /// let mut buf = [0u8; 12];
    /// dg.encode(&mut buf);
    /// ```
    pub fn encode(&self, buf: &mut [u8]) -> Result<usize>{
        let payload_size = self.body.len() + 1;

        if buf.len() < payload_size {
            return Err(SWError::BufferTooSmall);
        };

        buf[0] = self.header as u8;
        for i in 0..self.body.len() {
            buf[i+1] = self.body[i];
        };

        Ok(payload_size)
    }

    pub fn parse(buf: &[u8]) -> Result<Datagram> {
        if buf.len() < 2 {
            return Err(SWError::BufferTooSmall);
        };

        let header = DatagramHeader::from(buf[0])?;
        let body = &buf[1..];

        Ok(Datagram {
            header,
            body,
        })

    }
    
}

pub struct Window {
    pub size: usize,
    pub head: usize,
}