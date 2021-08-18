use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct LeverageFilter {
    pub min_leverage: f64,
    pub max_leverage: f64,
    pub leverage_step: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct PriceFilter {
    pub min_price: String,
    pub max_price: String,
    pub tick_size: String,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Default)]
pub struct LotSizeFilter {
    pub min_trading_qty: f64,
    pub max_trading_qty: f64,
    pub qty_step: f64,
}
