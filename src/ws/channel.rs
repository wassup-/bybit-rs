#[derive(Eq, PartialEq, Clone, Debug)]
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

impl Channel {
    /// Returns `true` if this channel requires authentication.
    pub fn requires_authentication(&self) -> bool {
        matches!(
            self,
            Channel::Position | Channel::Execution | Channel::Order | Channel::StopOrder
        )
    }
}
