use std::net::UdpSocket;

use sliding_window::SlidingWindow;

#[test]
pub fn main_test() {
    let socket = UdpSocket::bind("127.0.0.1:8080").unwrap();
    let mut sw = SlidingWindow::new(socket);

    sw.buffer_msg(b"Hello world");
}

pub fn main_test2() {
    let s1 = UdpSocket::bind("127.0.0.1:8080").unwrap();
    let mut sw1 = SlidingWindow::new(s1);

    let s2 = UdpSocket::bind("127.0.0.1:8080").unwrap();
    let mut sw2 = SlidingWindow::new(s2);

    sw1.buffer_msg(b"Message1");
    sw2.buffer_msg(b"Message2");
    sw1.buffer_msg(b"Message3");
    sw2.buffer_msg(b"Message4");
    sw1.buffer_msg(b"Message5");
    sw2.buffer_msg(b"Message6");
    sw1.buffer_msg(b"Message7");
    sw2.buffer_msg(b"Message8");
    
}