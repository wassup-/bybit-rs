use crate::{
    http::{Client, Query, Response},
    Result, Tickers,
};
use async_trait::async_trait;

#[async_trait]
pub trait FetchTickers {
    /// Fetch the tickers for a given symbol.
    /// * `client` - The client to use.
    async fn fetch_tickers(&self, symbol: &str) -> Result<Tickers>;
}

#[async_trait]
impl FetchTickers for Client {
    async fn fetch_tickers(&self, symbol: &str) -> Result<Tickers> {
        let query = query::Ticker {
            symbol: symbol.to_owned(),
        };
        let response: Response<Tickers> = self.get("/v2/public/tickers", &query).await?;
        Ok(response.result.unwrap_or_default())
    }
}

mod query {
    use super::Query;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Ticker {
        pub symbol: String,
    }

    impl Query for Ticker {}
}
