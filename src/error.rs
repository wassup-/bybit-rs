use super::{http, ws};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Http(http::Error),
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
