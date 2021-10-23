use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, Debug)]
pub enum TickDirection {
    /// Price rise
    PlusTick,
    /// Trade occurs at the same price as the previous trade,
    /// which occurred at a price higher than that for the trade preceding it
    ZeroPlusTick,
    /// Price drop
    MinusTick,
    /// Trade occurs at the same price as the previous trade,
    /// which occurred at a price lower than that for the trade preceding it
    ZeroMinusTick,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticker {
    /// Symbol
    pub symbol: String,
    /// Purchase price of the first order
    pub bid_price: String,
    /// Selling price of the first order
    pub ask_price: String,
    /// Latest transaction price
    pub last_price: String,
    /// Index price
    pub index_price: String,
    /// Mark price
    pub mark_price: String,
    /// Direction of price change
    pub last_tick_direction: TickDirection,
    /// Price of 24 hours ago
    pub prev_price_24h: String,
    /// Percentage change of market price relative to 24h
    pub price_24h_pcnt: String,
    /// Highest price in the last 24 hours
    pub high_price_24h: String,
    /// Lowest price in the last 24 hours
    pub low_price_24h: String,
    /// Hourly market price an hour ago
    pub prev_price_1h: String,
    /// Percentage change of market price relative to 1 hour ago
    pub price_1h_pcnt: String,
    /// Open interest
    pub open_interest: f64,
    /// Open position value
    pub open_value: String,
    /// Total turnover
    pub total_turnover: String,
    /// Turnover for 24h
    pub turnover_24h: String,
    /// Total volume
    pub total_volume: f64,
    /// Trading volume in the last 24 hours
    pub volume_24h: f64,
    /// Funding rate
    pub funding_rate: String,
    /// Predicted funding rate
    pub predicted_funding_rate: String,
    /// Next settlement time of capital cost
    pub next_funding_time: String,
    /// Countdown of settlement capital cost
    pub countdown_hour: i64,
    /// Delivery fee rate of Futures contract
    pub delivery_fee_rate: String,
    /// Predicted delivery price of Futures contract
    pub predicted_delivery_price: String,
    /// Delivery time of Futures contract
    pub delivery_time: String,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(transparent)]
pub struct Tickers {
    #[serde(rename = "result")]
    tickers: Vec<Ticker>,
}

impl Tickers {
    pub fn new(tickers: &[Ticker]) -> Self {
        Tickers {
            tickers: tickers.into(),
        }
    }

    /// Get the ticker for the given symbol.
    /// * `symbol` - The symbol to find the ticker for.
    pub fn get(&self, symbol: &str) -> Option<&Ticker> {
        let pos = self
            .tickers
            .iter()
            .position(|ticker| ticker.symbol == symbol)?;
        self.tickers.get(pos)
    }

    /// Returns an iterator over the tickers.
    pub fn tickers(&self) -> std::slice::Iter<'_, Ticker> {
        self.tickers.iter()
    }
}
