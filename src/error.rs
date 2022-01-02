use super::{http, ws};
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    Http(http::Error),
    Ws(ws::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
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
