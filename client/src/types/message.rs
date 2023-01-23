pub enum MessageHeader {}


pub struct Message {
    pub header: MessageHeader,
    pub sender: String,
    pub body: String,
}
