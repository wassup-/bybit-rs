use super::{http, ws};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Http error: {0}")]
    Http(http::Error),

    #[error("Ws error: {0}")]
    Ws(ws::Error),
}

impl From<http::Error> for Error {
    fn from(err: http::Error) -> Self {
        Self::Http(err)
    }
}

impl From<ws::Error> for Error {
    fn from(err: ws::Error) -> Self {
        Self::Ws(err)
    }
}
