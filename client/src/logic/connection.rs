use std::{net::UdpSocket, sync::Arc};

use crate::{error::ChatError, message::Message, client::Client};

struct Connection {
    socket: Arc<UdpSocket>,
    me: Client,
}

