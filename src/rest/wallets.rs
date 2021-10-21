use crate::{
    http::{Client, NoQuery, Query, Response, Result},
    Wallet, Wallets,
};
use async_trait::async_trait;

#[async_trait]
pub trait FetchWallets {
    /// Fetch all wallets.
    async fn fetch_wallets(&self) -> Result<Wallets>;
}

#[async_trait]
pub trait FetchWallet {
    /// Fetch the wallet for the given currency.
    /// * `currency` - The currency to fetch the wallet for.
    async fn fetch_wallet(&self, currency: &str) -> Result<Option<Wallet>>;
}

#[async_trait]
impl FetchWallets for Client {
    async fn fetch_wallets(&self) -> Result<Wallets> {
        let query = NoQuery::new();
        let query = self.sign_query(query);
        let response: Response<Wallets> = self.get("/v2/private/wallet/balance", &query).await?;
        response.result()
    }
}

#[async_trait]
impl FetchWallet for Client {
    async fn fetch_wallet(&self, currency: &str) -> Result<Option<Wallet>> {
        let query = query::Wallet {
            coin: currency.to_owned(),
        };
        let query = self.sign_query(query);
        let response: Response<Wallets> = self.get("/v2/private/wallet/balance", &query).await?;
        response.result().map(|res| res.get(currency).cloned())
    }
}

mod query {
    use super::Query;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Wallet {
        pub coin: String,
    }

    impl Query for Wallet {}
}
