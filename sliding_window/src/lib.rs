pub mod types;
pub mod error;

use error::*;
use types::*;

use std::net::UdpSocket;

/// Sliding window safely sends and receives messages trough Udp connection
pub struct SlidingWindow<'a> {
    pub socket: UdpSocket,
    pub buffer: Vec<Datagram<'a>>,
    pub window: Window,
}

impl<'a> SlidingWindow<'a> {
    /// Constructs a new `SlidingWindow`.
    /// 
    /// Default size of window is 4 (for custom size use `SlidingWindow::from`)
    /// 
    /// # Examples
    /// ```rust
    /// use sliding_window::SlidingWindow;
    /// use std::net::UdpSocket;
    /// let socket = UdpSocket::bind("127.0.0.1:8000").unwrap();
    /// let sw = SlidingWindow::new(socket);
    /// ```
    pub fn new(socket: UdpSocket) -> Self {
        Self {
            socket,
            buffer: Vec::new(),
            window: Window {
                size: 4,
                head: 0,
            },
        }
    }

    /// Constructs a new `SlidingWindow` with specified size of window.
    /// 
    /// # Examples
    /// ```rust
    /// use sliding_window::SlidingWindow;
    /// use std::net::UdpSocket;
    /// let socket = UdpSocket::bind("127.0.0.1:8000").unwrap();
    /// let sw = SlidingWindow::from(socket, 3);
    /// ```
    pub fn from(socket: UdpSocket, window_size: usize) -> Self {
        Self {
            socket,
            buffer: Vec::new(),
            window: Window {
                size: window_size,
                head: 0,
            },
        }
    }

    /// Adds a message to buffer, does not send anything until `SlidingWindow::flush` is called
    /// 
    /// # Example
    /// ```rust
    /// use sliding_window::SlidingWindow;
    /// use std::net::UdpSocket;
    /// let socket = UdpSocket::bind("127.0.0.1:8000").unwrap();
    /// let mut sw = SlidingWindow::new(socket);
    /// 
    /// let msg = b"Hello world!";
    /// 
    /// sw.buffer_msg(msg);
    /// ```
    pub fn buffer_msg(&mut self, msg: &'a [u8]) {
        let datagram = Datagram {
            header: DatagramHeader::SEND,
            body: msg,
        };

        self.buffer.push(datagram);
    }

    pub fn flush(&mut self) -> Result<()> {
        let window = &mut self.window;
        let buffer = &self.buffer;
        let socket = &self.socket;

        let mut stop = false;

        while !stop {
            for i in 0..window.size {
                let dg = match buffer.get(window.head + i) {
                    Some(dg) => dg,
                    None => {
                        stop = true;    
                        &Datagram { 
                            header: DatagramHeader::FIN, 
                            body: &[]
                        }
                    }
                };
                let mut buf = vec![0u8; 256];
                let bytes_read = dg.encode(&mut buf)?;
    
                socket.send(&buf[..bytes_read])?;
            };

            let mut buf = [0u8; 256];
            loop {
                let bytes_read = socket.recv(&mut buf)?;

                if bytes_read == 0 {
                    continue;
                }

                let dg = Datagram::parse(&buf)?;
                if let DatagramHeader::ACK = dg.header {
                    let ack_num = dg.body[0];
                    window.head = ack_num as usize;
                } 
                
            }
 
        };

        Ok(())
    }

    pub fn check_for_messages(&mut self) -> Result<Option<Vec<&'a [u8]>>> {
        let socket = &self.socket;
        let buffer = &mut self.buffer;
        let window = &mut self.window;

        let mut buf = [0u8; 256];
        loop {
            let bytes_read = socket.recv(&mut buf)?;

            if bytes_read == 0 {
                continue;
            };

            let dg = Datagram::parse(&buf)?;
        };
        
    }
}

