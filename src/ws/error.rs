use super::Channel;
use tungstenite::error::Error as WsError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Ws(WsError),
    Json(serde_json::Error),
    NotConnected,
    NotAuthenticated,
    MissingSubscriptionConfirmation(Channel),
    SubscriptionFailed(Channel),
    NotSubscribed(Channel),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Channel: {}", self)
    }
}

impl From<WsError> for Error {
    fn from(err: WsError) -> Self {
        Self::Ws(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
