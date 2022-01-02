use super::{http, ws};
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Http Error: {0}")]
    Http(http::Error),
    #[error("Ws Error: {0}")]
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
