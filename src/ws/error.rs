use super::Channel;
use tungstenite::error::Error as WsError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    Ws(Error),
    Json(serde_json::Error),
    NotConnected,
    NotAuthenticated,
    MissingSubscriptionConfirmation(Channel),
    SubscriptionFailed(Channel),
    NotSubscribed(Channel),
}

impl From<Error> for Error {
    fn from(err: Error) -> Self {
        Self::Ws(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}
