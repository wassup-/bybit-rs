use crate::{
    http::{Client, NoQuery, Query, Response, Result},
    Wallet, WalletFundRecord, WalletFundType, WalletWithdrawRecord, Wallets, WithdrawStatus,
};
use async_trait::async_trait;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct FetchWalletFundRecordsOptions {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub currency: Option<String>,
    pub wallet_fund_type: Option<WalletFundType>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Default, Serialize)]
pub struct FetchWalletWithdrawRecordsOptions {
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub coin: Option<String>,
    pub status: Option<WithdrawStatus>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

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
pub trait FetchWalletFundRecords {
    /// Fetch the wallet fund records with the given options.
    /// * `options` - The options for fetching the fund records.
    async fn fetch_wallet_fund_records(
        &self,
        options: FetchWalletFundRecordsOptions,
    ) -> Result<Vec<WalletFundRecord>>;
}

#[async_trait]
pub trait FetchWalletWithdrawRecords {
    /// Fetch the wallet withdraw records with the given options.
    /// * `options` - The options for fetching the withdraw records.
    async fn fetch_wallet_withdraw_records(
        &self,
        options: FetchWalletWithdrawRecordsOptions,
    ) -> Result<Vec<WalletWithdrawRecord>>;
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

#[async_trait]
impl FetchWalletFundRecords for Client {
    async fn fetch_wallet_fund_records(
        &self,
        options: FetchWalletFundRecordsOptions,
    ) -> Result<Vec<WalletFundRecord>> {
        let query = self.sign_query(options);
        let response: Response<response::WalletFundRecords> =
            self.get("/v2/private/wallet/fund/records", &query).await?;
        response.result().map(|res| res.data)
    }
}

#[async_trait]
impl FetchWalletWithdrawRecords for Client {
    async fn fetch_wallet_withdraw_records(
        &self,
        options: FetchWalletWithdrawRecordsOptions,
    ) -> Result<Vec<WalletWithdrawRecord>> {
        let query = self.sign_query(options);
        let response: Response<response::WalletWithdrawRecords> =
            self.get("/v2/private/wallet/withdraw/list", &query).await?;
        response.result().map(|res| res.data.unwrap_or_default())
    }
}

impl Query for FetchWalletFundRecordsOptions {}
impl Query for FetchWalletWithdrawRecordsOptions {}

mod query {
    use super::Query;
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct Wallet {
        pub coin: String,
    }

    impl Query for Wallet {}
}

mod response {
    use super::{WalletFundRecord, WalletWithdrawRecord};
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct WalletFundRecords {
        pub data: Vec<WalletFundRecord>,
    }

    #[derive(Deserialize)]
    pub struct WalletWithdrawRecords {
        pub data: Option<Vec<WalletWithdrawRecord>>,
    }
}
