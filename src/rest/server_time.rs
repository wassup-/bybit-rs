use crate::http::{Client, Ignore, NoQuery, Response, Result};
use async_trait::async_trait;

#[async_trait]
pub trait ServerTime {
    async fn server_time(&self) -> Result<String>;
}

#[async_trait]
impl ServerTime for Client {
    async fn server_time(&self) -> Result<String> {
        let query = NoQuery::new();
        let response: Response<Ignore> = self.get("/v2/public/time", &query).await?;
        Ok(response.time_now)
    }
}
