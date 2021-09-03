use super::Result;

pub type Message = tungstenite::protocol::Message;

pub trait IntoMessage {
    fn into_message(self) -> Result<Message>;
}

impl IntoMessage for Message {
    fn into_message(self) -> Result<Message> {
        Ok(self)
    }
}
