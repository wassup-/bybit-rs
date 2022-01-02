use super::Response;
use thiserror::Error as ThisError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, ThisError)]
pub enum Error {
    Url(url::ParseError),
    Reqwest(reqwest::Error),
    ErrorCode(ErrorCode),
}

#[derive(Debug)]
pub struct ErrorCode {
    pub code: i64,
    pub msg: String,
    pub ext_code: String,
    pub ext_info: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Self::Url(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<ErrorCode> for Error {
    fn from(code: ErrorCode) -> Self {
        Self::ErrorCode(code)
    }
}

impl<T> From<Response<T>> for Error {
    fn from(res: Response<T>) -> Self {
        Self::ErrorCode(res.into())
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T> From<Response<T>> for ErrorCode {
    fn from(res: Response<T>) -> Self {
        ErrorCode {
            code: res.ret_code,
            msg: res.ret_msg,
            ext_code: res.ext_code,
            ext_info: res.ext_info,
        }
    }
}
