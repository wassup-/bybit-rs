use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Response<T> {
    pub result: Option<T>,
    pub ret_code: i64,
    pub ret_msg: String,
    pub ext_code: String,
    pub ext_info: String,
    pub time_now: String,
    pub rate_limit_status: Option<i64>,
    pub rate_limit_reset_ms: Option<i64>,
    pub rate_limit: Option<i64>,
}
