use crate::{deserialize::string_or_number, UserId};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum WalletFundType {
    Deposit,
    Withdraw,
    #[serde(rename = "RealisedPNL")]
    RealisedPnl,
    Commission,
    Refund,
    Prize,
    ExchangeOrderWithdraw,
    ExchangeOrderDeposit,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum WithdrawStatus {
    ToBeConfirmed,
    UnderReview,
    Pending,
    Success,
    CancelByUser,
    Reject,
    Expire,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct WalletId(i64);

#[derive(Deserialize, Debug, Copy, Clone, Default)]
pub struct Wallet {
    /// User equity
    pub equity: f64,
    /// Available balance (wallet balance - used margin)
    pub available_balance: f64,
    /// Used margin
    pub used_margin: f64,
    /// Pre-occupied order margin
    pub order_margin: f64,
    /// Position margin
    pub position_margin: f64,
    /// Position closing fee occupied
    pub occ_closing_fee: f64,
    /// Pre-occupied funding fee
    pub occ_funding_fee: f64,
    /// Wallet data endpoints
    pub wallet_balance: f64,
    /// Today's realised pnl
    pub realised_pnl: f64,
    /// Today's unrealised pnl
    pub unrealised_pnl: f64,
    /// Accumulated realised pnl (all-time)
    pub cum_realised_pnl: f64,
    /// Experience gold
    pub given_cash: f64,
    /// Service cash is used for user's service charge
    pub service_cash: f64,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct WalletFundRecordId(i64);

#[derive(Deserialize, Debug, Clone)]
pub struct WalletFundRecord {
    pub id: WalletFundRecordId,
    pub user_id: UserId,
    pub coin: String,
    pub wallet_id: WalletId,
    #[serde(rename = "type")]
    pub fund_type: WalletFundType,
    #[serde(deserialize_with = "string_or_number")]
    pub amount: f64,
    pub tx_id: String,
    pub address: String,
    #[serde(deserialize_with = "string_or_number")]
    pub wallet_balance: f64,
    pub exec_time: String,
    pub cross_seq: i64,
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct WalletWithdrawRecordId(i64);

#[derive(Deserialize, Debug, Clone)]
pub struct WalletWithdrawRecord {
    pub id: WalletWithdrawRecordId,
    pub user_id: UserId,
    pub coin: String,
    pub status: WithdrawStatus,
    #[serde(deserialize_with = "string_or_number")]
    pub amount: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub fee: f64,
    pub address: String,
    pub tx_id: String,
    pub submited_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(transparent)]
pub struct AssetExchangeRecordId(i64);

#[derive(Deserialize, Debug, Clone)]
pub struct AssetExchangeRecord {
    pub id: AssetExchangeRecordId,
    pub exchange_rate: f64,
    pub from_coin: String,
    pub to_coin: String,
    pub to_amount: f64,
    pub from_fee: f64,
    pub from_amount: f64,
    pub created_at: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct Wallets(BTreeMap<String, Wallet>);

impl Wallets {
    /// Returns an iterator over the currencies.
    pub fn currencies(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    /// Get the wallet for the given currency.
    /// * `currency` - The currency to find the wallet for.
    pub fn get(&self, currency: &str) -> Option<&Wallet> {
        self.0.get(currency)
    }

    /// Get the wallet for the given currency.
    /// * `currency` - The currency to find the wallet for.
    pub fn get_mut(&mut self, currency: &str) -> Option<&mut Wallet> {
        self.0.get_mut(currency)
    }
}

impl std::fmt::Display for WalletId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for WalletFundRecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for WalletWithdrawRecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for AssetExchangeRecordId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
    #[test]
    fn get_wallet() {
        let mut wallets: BTreeMap<String, Wallet> = BTreeMap::new();
        wallets.insert(
            "BTC".to_owned(),
            Wallet {
                equity: 1.3,
                available_balance: 35624.5,
                ..Default::default()
            },
        );
        let wallets = Wallets(wallets);
        assert!(wallets.get("BTC").is_some());
        assert!(wallets.get("ETH").is_none());
        assert_eq!(wallets.get("BTC").unwrap().equity, 1.3);
        assert_eq!(wallets.get("BTC").unwrap().available_balance, 35624.5);
    }
}
