use thiserror::Error as ThisError;

#[derive(Eq, PartialEq, Clone, Debug, ThisError)]
pub enum Channel {
    // Public
    OrderBook25(String),
    OrderBook200(String),
    Trade,
    Insurance,
    InstrumentInfo(String),
    KlineV2(String, String),
    Liquidation,

    // Private
    Position,
    Execution,
    Order,
    StopOrder,
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Channel {
    /// Returns `true` if this channel requires authentication.
    pub fn requires_authentication(&self) -> bool {
        matches!(
            self,
            Channel::Position | Channel::Execution | Channel::Order | Channel::StopOrder
        )
    }
}
