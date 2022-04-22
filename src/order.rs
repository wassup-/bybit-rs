use crate::deserialize::{optional_string_or_number, string_or_number};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct OrderId(String);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct OrderLinkId(String);

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ExecId(String);

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum OrderType {
    Limit,
    Market,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum TimeInForce {
    GoodTillCancel,
    ImmediateOrCancel,
    FillOrKill,
    PostOnly,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum TriggerPrice {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    LastPrice,
    IndexPrice,
    MarkPrice,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum OrderStatus {
    /// Order has been accepted by the system but not yet put through the matching engine
    Created,
    Rejected,
    /// Order has been placed successfuly
    New,
    PartiallyFilled,
    Filled,
    Cancelled,
    /// Matching engine has received the cancelation request but it may not be canceled successfuly
    PendingCancel,
    Untriggered,
    Triggered,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum StopOrderStatus {
    /// Order has been triggered and the new active order has been successfuly placed.
    /// Is the final state of a successful conditional order
    Active,
    /// Order yet to be triggered
    Untriggered,
    /// Order has been triggered by last traded price
    Triggered,
    /// Order has been canceled successfuly
    Cancelled,
    /// Order has been triggered but failed to be placed (e.g. due to insufficient margin)
    Rejected,
    /// Order has been canceled by the user before being triggered
    Deactivated,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum CancelType {
    #[serde(rename = "UNKNOWN")]
    Unknown,
    CancelByUser,
    CancelByReduceOnly,
    /// Canceled due to liquidation
    CancelByPrepareLiq,
    /// Canceled due to liquidation
    CancellAllBeforeLiq,
    /// Canceled due to ADL
    CancelByPrepareAdl,
    /// Canceled due to ADL
    CancelAllBeforeAdl,
    CancelByAdmin,
    /// TP/SL order canceled successfuly
    CancelByTpSlTsClear,
    /// Canceled after TP/SL is triggered
    CancelByPzSideCh,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum CreateType {
    CreateByUser,
    CreateByClosing,
    CreateByAdminClosing,
    CreateByStopOrder,
    CreateByTrailingStop,
    CreateByTakeProfit,
    CreateByStopLoss,
    /// Created by partial liquidation
    CreateByLiq,
    /// Created by ADL
    #[serde(rename = "CreateByAdl_PassThrough")]
    CreateByAdlPassthrough,
    /// Created by liquidation takeover
    #[serde(rename = "CreateByTakeOver_PassThrough")]
    CreateByTakeOverPassthrough,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum ExecType {
    Trade,
    AdlTrade,
    Funding,
    BustTrade,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum LiquidityType {
    /// Liquidity maker
    AddedLiquidity,
    /// Liquidity taker
    RemovedLiquidity,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum TpSlMode {
    /// Full take profit / stop loss mode.
    /// A single TP order and a single SL order can be placed, covering the entire position.
    Full,
    /// Partial take profit / stop loss mode.
    /// Multiple TP and SL orders can be placed, covering portions of the position.
    Partial,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
pub enum StopOrderType {
    TakeProfit,
    StopLoss,
    TrailingStop,
    Stop,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Sort {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(transparent)]
pub struct UserId(i64);

#[derive(Serialize, Deserialize, Copy, Clone, Debug, Eq, PartialEq)]
#[serde(transparent)]
pub struct LiquidatedOrderId(i64);

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LiquidatedOrder {
    pub id: LiquidatedOrderId,
    pub symbol: String,
    pub side: Side,
    pub qty: f64,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
    /// Unique order id
    #[serde(rename = "order_id")]
    pub id: OrderId,
    /// User id
    pub user_id: UserId,
    /// Customised order id
    #[serde(rename = "order_link_id")]
    pub link_id: OrderLinkId,
    /// Order price
    #[serde(deserialize_with = "string_or_number")]
    pub price: f64,
    /// Order quantity in USD.
    #[serde(deserialize_with = "string_or_number")]
    pub qty: f64,
    /// Symbol
    pub symbol: String,
    /// Side
    pub side: Side,
    /// Order status
    pub order_status: OrderStatus,
    /// Order type
    pub order_type: OrderType,
    /// Last execution time
    #[serde(deserialize_with = "string_or_number")]
    pub last_exec_time: f64,
    /// Last execution price
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub last_exec_price: Option<f64>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Trigger scenario for single action
    pub create_type: Option<CreateType>,
    /// Trigger scenario for cancel operation
    pub cancel_type: Option<CancelType>,
    /// Number of unfilled contracts (from the order's size)
    #[serde(deserialize_with = "string_or_number")]
    pub leaves_qty: f64,
    /// The estimated value corresponding to the number of remaining orders.
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub leaves_value: Option<f64>,
    /// Cumulative qty of trading
    #[serde(deserialize_with = "string_or_number")]
    pub cum_exec_qty: f64,
    /// Cumulative value of trading
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub cum_exec_value: Option<f64>,
    /// Cumulative trading fees
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub cum_exec_fee: Option<f64>,
    /// The reason the order was rejected
    pub reject_reason: String,
    /// Creation time
    pub created_at: String,
    /// Update time
    pub updated_at: String,
    /// Take profit price
    #[serde(deserialize_with = "string_or_number")]
    pub take_profit: f64,
    /// Stop loss price
    #[serde(deserialize_with = "string_or_number")]
    pub stop_loss: f64,
    /// Take profit trigger price type
    pub tp_trigger_by: TriggerPrice,
    /// Stop loss trigger price type
    pub sl_trigger_by: TriggerPrice,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LinearOrder {
    /// Unique order id
    #[serde(rename = "order_id")]
    pub id: OrderId,
    /// User id
    pub user_id: UserId,
    /// Customised order id
    #[serde(rename = "order_link_id")]
    pub link_id: OrderLinkId,
    /// Order price
    #[serde(deserialize_with = "string_or_number")]
    pub price: f64,
    /// Order quantity in USD.
    #[serde(deserialize_with = "string_or_number")]
    pub qty: f64,
    /// Symbol
    pub symbol: String,
    /// Side
    pub side: Side,
    /// Order status
    pub order_status: OrderStatus,
    /// Order type
    pub order_type: OrderType,
    /// Last execution price
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub last_exec_price: Option<f64>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Cumulative qty of trading
    #[serde(deserialize_with = "string_or_number")]
    pub cum_exec_qty: f64,
    /// Cumulative value of trading
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub cum_exec_value: Option<f64>,
    /// Cumulative trading fees
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub cum_exec_fee: Option<f64>,
    /// Reduce only means close order, false means open position
    pub reduce_only: bool,
    /// Can only reduce your position
    pub close_on_trigger: bool,
    /// Creation time
    pub created_time: String,
    /// Update time
    pub updated_time: String,
    /// Take profit price
    #[serde(deserialize_with = "string_or_number")]
    pub take_profit: f64,
    /// Stop loss price
    #[serde(deserialize_with = "string_or_number")]
    pub stop_loss: f64,
    /// Take profit trigger price type
    pub tp_trigger_by: TriggerPrice,
    /// Stop loss trigger price type
    pub sl_trigger_by: TriggerPrice,
    /// Position idx
    pub position_idx: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ConditionalOrder {
    /// Conditional order ID. Once triggered the condition order creates active order with same ID
    #[serde(rename = "stop_order_id")]
    pub id: OrderId,
    /// User id
    pub user_id: UserId,
    /// Customised order id
    #[serde(rename = "order_link_id")]
    pub link_id: OrderLinkId,
    /// Order price
    #[serde(deserialize_with = "string_or_number")]
    pub price: f64,
    /// Order quantity in USD.
    #[serde(deserialize_with = "string_or_number")]
    pub qty: f64,
    /// Symbol
    pub symbol: String,
    /// Side
    pub side: Side,
    /// Order status
    pub order_status: OrderStatus,
    /// Order type
    pub order_type: OrderType,
    /// Last execution price
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub last_exec_price: Option<f64>,
    /// Time in force
    pub time_in_force: TimeInForce,
    /// Reduce only
    pub reduce_only: bool,
    /// Can only reduce your position
    pub close_on_trigger: bool,
    /// Creation time
    pub created_time: Option<String>,
    /// Update time
    pub updated_time: Option<String>,
    /// Update at (for inverse)
    pub updated_at: Option<String>,
    /// Take profit price
    #[serde(deserialize_with = "string_or_number")]
    pub take_profit: f64,
    /// Stop loss price
    #[serde(deserialize_with = "string_or_number")]
    pub stop_loss: f64,
    /// Take profit trigger price type
    pub tp_trigger_by: TriggerPrice,
    /// Stop loss trigger price type
    pub sl_trigger_by: TriggerPrice,
    /// Position idx
    pub position_idx: i64,
    /// Order trigger price type
    pub trigger_by: TriggerPrice,
    /// Market price at placing order
    pub base_price: String,
    /// Remark
    pub remark: Option<String>,
    /// Reject reason (maybe change to enum later? bad api docs)
    pub reject_reason: Option<String>,
    /// Trigger price
    pub stop_px: Option<String>,
}

impl std::fmt::Display for OrderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for OrderLinkId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for ExecId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Display for LiquidatedOrderId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for Side {
    fn default() -> Self {
        Side::Buy
    }
}

impl Default for OrderType {
    fn default() -> Self {
        OrderType::Market
    }
}

impl Default for TimeInForce {
    fn default() -> Self {
        TimeInForce::GoodTillCancel
    }
}
