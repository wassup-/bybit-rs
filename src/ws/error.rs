use super::Channel;
use tungstenite::error::Error as WsError;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    Ws(WsError),
    Json(serde_json::Error),
    NotConnected,
    NotAuthenticated,
    MissingSubscriptionConfirmation(Channel),
    SubscriptionFailed(Channel),
    NotSubscribed(Channel),
}

impl From<WsError> for Error {
    fn from(err: WsError) -> Self {
        Self::Ws(err)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Bob: {}", self.0)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
