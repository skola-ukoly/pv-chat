use crate::model::{
    net::NetState,
    ui_state::UiState,

};

pub struct ChatState {
    pub net: NetState,
    pub ui: UiState,
    pub cipher: CipherState,
}