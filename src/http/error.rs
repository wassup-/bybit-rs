use super::Response;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Transport(reqwest::Error),
    ErrorCode(ErrorCode),
}

#[derive(Debug)]
pub struct ErrorCode {
    pub code: i64,
    pub msg: String,
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Transport(err)
    }
}

impl<T> From<Response<T>> for Error {
    fn from(res: Response<T>) -> Self {
        let code = ErrorCode {
            code: res.ret_code,
            msg: res.ret_msg,
        };
        Self::ErrorCode(code)
    }
}
