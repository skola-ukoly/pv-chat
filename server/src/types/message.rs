pub struct Message<'a> {
    pub sender: String,
    pub body: &'a [u8],
}

