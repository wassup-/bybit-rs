use crate::{
    http::{Client, Query, Response},
    Result, Symbols,
};
use async_trait::async_trait;

#[async_trait]
pub trait FetchSymbols {
    /// Fetch all symbols.
    async fn fetch_symbols(&self) -> Result<Symbols>;
}

#[async_trait]
impl FetchSymbols for Client {
    async fn fetch_symbols(&self) -> Result<Symbols> {
        let query = query::Symbols;
        let response: Response<Symbols> = self.get("/v2/public/symbols", &query).await?;
        Ok(response.result.unwrap_or_default())
    }
}

mod query {
    use super::Query;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Symbols;

    impl Query for Symbols {}
}
