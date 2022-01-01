use super::Response;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    Url(url::ParseError),
    Reqwest(reqwest::Error),
    ErrorCode(ErrorCode),
}

#[derive(Debug, Error)]
pub struct ErrorCode {
    pub code: i64,
    pub msg: String,
    pub ext_code: String,
    pub ext_info: String,
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
