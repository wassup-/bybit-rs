use super::Channel;
use tungstenite::error::Error as WsError;
use thiserror::Error as ThisError;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("WsError: {0}")]
    Ws(WsError),
    #[error("JsonError: {0}")]
    Json(serde_json::Error),
    #[error("NotConnected")]
    NotConnected,
    #[error("NotAuthenticated")]
    NotAuthenticated,
    #[error("MissingSubscriptionConfirmation: {0}")]
    MissingSubscriptionConfirmation(Channel),
    #[error("SubscriptionFailed: {0}")]
    SubscriptionFailed(Channel),
    #[error("NotSubscribed: {0}")]
    NotSubscribed(Channel),
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
