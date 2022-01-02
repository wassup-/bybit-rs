use super::Channel;
use tungstenite::error::Error as WsError;
use thiserror::Error as ThisError;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Ws Error: {0:?}")]
    Ws(WsError),
    #[error("Json Error: {0:?}")]
    Json(serde_json::Error),
    #[error("Not Connected")]
    NotConnected,
    #[error("Not Authenticated")]
    NotAuthenticated,
    #[error("Missing Subscription Confirmation: {0:?}")]
    MissingSubscriptionConfirmation(Channel),
    #[error("Subscription Failed: {0:?}")]
    SubscriptionFailed(Channel),
    #[error("Not Subscribed: {0:?}")]
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
