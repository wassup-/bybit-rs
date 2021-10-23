use crate::deserialize::{optional_string_or_number, string_or_number};
use crate::{
    order::Side, ticker::TickDirection, trade::TradeId, CancelType, CreateType, ExecId, ExecType,
    OrderId, OrderLinkId, OrderStatus, OrderType, StopOrderStatus, StopOrderType, TimeInForce,
    TriggerPrice, UserId,
};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Data {
    OrderbookSnapshot(OrderbookSnapshot),
    OrderbookDelta(OrderbookDelta),
    Trade(Trade),
    Insurance(Insurance),
    InstrumentInfoSnapshot(InstrumentInfoSnapshot),
    InstrumentInfoDelta(InstrumentInfoDeltaData),
    KlineV2(KlineV2),
    Liquidation(Liquidation),
    Position(Position),
    Execution(Execution),
    Order(Order),
    StopOrder(StopOrder),
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderbookSnapshot {
    pub id: i64,
    #[serde(deserialize_with = "string_or_number")]
    pub price: f64,
    pub symbol: String,
    pub side: Side,
    pub size: Option<i64>,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct OrderbookSnapshotResponse {
    pub topic: String,
    pub data: Vec<OrderbookSnapshot>,
    pub cross_seq: i64,
    pub timestamp_e6: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Trade {
    pub side: Side,
    pub size: i64,
    pub symbol: String,
    pub price: f64,
    pub tick_direction: TickDirection,
    pub trade_id: TradeId,
    pub timestamp: String,
    pub trade_time_ms: i64,
    pub cross_seq: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct TradeResponse {
    pub topic: String,
    pub data: Vec<Trade>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Insurance {
    pub currency: String,
    pub timestamp: String,
    pub wallet_balance: f64,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct InsuranceResponse {
    pub topic: String,
    pub data: Vec<Insurance>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstrumentInfoSnapshot {
    pub id: i64,
    pub symbol: String,
    pub last_price_e4: i64,
    pub bid1_price_e4: i64,
    pub ask1_price_e4: i64,
    pub last_tick_direction: TickDirection,
    pub prev_price_24h_e4: i64,
    pub prev_24h_pcnt_e4: i64,
    pub high_price_24h_e4: i64,
    pub low_price_24h_e4: i64,
    pub prev_price_1h_e4: i64,
    pub price_1h_pcnt_e4: i64,
    pub mark_price_e4: i64,
    pub index_price_e4: i64,
    pub open_interest: i64,
    pub open_value_e8: i64,
    pub total_turnover_e8: i64,
    pub turnover_24h_e8: i64,
    pub total_volume: i64,
    pub volume_24h: i64,
    pub funding_rate_e6: i64,
    pub predicted_funding_rate_e6: i64,
    pub cross_seq: i64,
    pub created_at: String,
    pub updated_at: String,
    pub next_funding_time: String,
    pub countdown_hour: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstrumentInfoDelta {
    pub id: i64,
    pub symbol: String,
    pub prev_price_24h_e4: i64,
    pub prev_24h_pcnt_e4: i64,
    pub open_value_e8: i64,
    pub total_turnover_e8: i64,
    pub turnover_24h_e8: i64,
    pub volume_24h: i64,
    pub cross_seq: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KlineV2 {
    pub start: i64,
    pub end: i64,
    pub open: f64,
    pub close: f64,
    pub high: f64,
    pub low: f64,
    pub volume: f64,
    pub turnover: f64,
    pub confirm: bool,
    pub cross_seq: i64,
    pub timestamp: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OrderbookDelta {
    pub delete: Vec<OrderbookSnapshot>,
    pub update: Vec<OrderbookSnapshot>,
    pub insert: Vec<OrderbookSnapshot>,
    #[serde(rename = "transactTimeE6")]
    pub transaction_time_e6: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct OrderbookDeltaResponse {
    pub topic: String,
    pub data: OrderbookDelta,
    pub cross_seq: i64,
    pub timestamp_e6: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct InstrumentInfoSnapshotResponse {
    pub topic: String,
    pub data: InstrumentInfoSnapshot,
    pub cross_seq: i64,
    pub timestamp_e6: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct InstrumentInfoDeltaData {
    pub delete: Vec<InstrumentInfoDelta>,
    pub update: Vec<InstrumentInfoDelta>,
    pub insert: Vec<InstrumentInfoDelta>,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct InstrumentInfoDeltaResponse {
    pub topic: String,
    pub data: InstrumentInfoDeltaData,
    pub cross_seq: i64,
    pub timestamp_e6: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct KlineV2Response {
    pub topic: String,
    pub data: Vec<KlineV2>,
    pub timestamp_e6: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ResponseType {
    Snapshot,
    Delta,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Request {
    pub op: String,
    pub args: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct RequestResponse {
    pub success: bool,
    pub ret_msg: String,
    pub conn_id: String,
    pub request: Request,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PositionAction {
    Update,
}

#[derive(Deserialize, Debug, Clone)]
pub enum PositionStatus {
    Normal,
    Liq,
    Adl,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Position {
    pub user_id: UserId,
    pub symbol: String,
    pub size: i64,
    pub side: Side,
    pub position_value: f64,
    pub entry_price: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub liq_price: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub bust_price: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub leverage: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub order_margin: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub position_margin: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub available_balance: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub take_profit: f64,
    pub tp_trigger_by: TriggerPrice,
    #[serde(deserialize_with = "string_or_number")]
    pub stop_loss: f64,
    pub sl_trigger_by: TriggerPrice,
    #[serde(deserialize_with = "string_or_number")]
    pub realised_pnl: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub trailing_stop: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub trailing_active: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub wallet_balance: f64,
    pub risk_id: i64,
    #[serde(deserialize_with = "string_or_number")]
    pub occ_closing_fee: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub occ_funding_fee: f64,
    pub auto_add_margin: i64,
    #[serde(deserialize_with = "string_or_number")]
    pub cum_realised_pnl: f64,
    pub position_status: PositionStatus,
    pub position_seq: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct PositionResponse {
    pub topic: String,
    pub action: PositionAction,
    pub data: Vec<Position>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Execution {
    pub symbol: String,
    pub side: Side,
    pub order_id: OrderId,
    pub exec_id: ExecId,
    pub order_link_id: OrderLinkId,
    #[serde(deserialize_with = "string_or_number")]
    pub price: f64,
    pub order_qty: i64,
    pub exec_type: ExecType,
    pub exec_qty: i64,
    #[serde(deserialize_with = "string_or_number")]
    pub exec_fee: f64,
    pub leaves_qty: i64,
    pub is_maker: bool,
    pub trade_time: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct ExecutionResponse {
    pub topic: String,
    pub data: Vec<Execution>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Order {
    #[serde(rename = "order_id")]
    pub id: OrderId,
    #[serde(rename = "order_link_id")]
    pub link_id: OrderLinkId,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    #[serde(deserialize_with = "string_or_number")]
    pub price: f64,
    pub qty: i64,
    pub time_in_force: TimeInForce,
    pub create_type: Option<CreateType>,
    pub cancel_type: Option<CancelType>,
    pub order_status: OrderStatus,
    #[serde(deserialize_with = "string_or_number")]
    pub leaves_qty: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub cum_exec_qty: f64,
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub cum_exec_value: Option<f64>,
    #[serde(deserialize_with = "optional_string_or_number", default)]
    pub cum_exec_fee: Option<f64>,
    #[serde(deserialize_with = "string_or_number")]
    pub take_profit: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub stop_loss: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub trailing_stop: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub trailing_active: f64,
    pub reduce_only: bool,
    pub close_on_trigger: bool,
    pub timestamp: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct OrderResponse {
    pub topic: String,
    pub data: Vec<Order>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct StopOrder {
    pub order_id: OrderId,
    pub order_link_id: OrderLinkId,
    pub user_id: UserId,
    pub symbol: String,
    pub side: Side,
    pub order_type: OrderType,
    #[serde(deserialize_with = "string_or_number")]
    pub price: f64,
    pub qty: i64,
    pub time_in_force: TimeInForce,
    pub create_type: CreateType,
    pub cancel_type: CancelType,
    pub order_status: StopOrderStatus,
    pub stop_order_type: StopOrderType,
    pub trigger_by: TriggerPrice,
    #[serde(deserialize_with = "string_or_number")]
    pub trigger_price: f64,
    pub close_on_trigger: bool,
    pub timestamp: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct StopOrderResponse {
    pub topic: String,
    pub data: Vec<StopOrder>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Liquidation {
    pub symbol: String,
    pub side: Side,
    #[serde(deserialize_with = "string_or_number")]
    pub price: f64,
    #[serde(deserialize_with = "string_or_number")]
    pub qty: f64,
    pub time: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub(super) struct LiquidationResponse {
    pub topic: String,
    pub data: Liquidation,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub(super) enum Response {
    Request(RequestResponse),
    OrderbookSnapshot(OrderbookSnapshotResponse),
    OrderbookDelta(OrderbookDeltaResponse),
    Trade(TradeResponse),
    Insurance(InsuranceResponse),
    InstrumentInfoSnapshot(InstrumentInfoSnapshotResponse),
    InstrumentInfoDelta(InstrumentInfoDeltaResponse),
    KlineV2(KlineV2Response),
    Liquidation(LiquidationResponse),
    Position(PositionResponse),
    Execution(ExecutionResponse),
    Order(OrderResponse),
    StopOrder(StopOrderResponse),
}
