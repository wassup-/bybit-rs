use crate::{
    http::{Client, Query, Response, Result},
    order::*,
};
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListActiveOrdersFilter {
    pub symbol: String,
    pub order_status: Option<OrderStatus>,
    pub direction: Option<String>,
    pub limit: Option<i64>,
    pub cursor: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ActiveOrderId {
    OrderId(OrderId),
    OrderLinkId(OrderLinkId),
}

impl ActiveOrderId {
    pub fn order_id(&self) -> Option<OrderId> {
        match self {
            ActiveOrderId::OrderId(order_id) => Some(order_id.clone()),
            _ => None,
        }
    }

    pub fn order_link_id(&self) -> Option<OrderLinkId> {
        match self {
            ActiveOrderId::OrderLinkId(order_link_id) => Some(order_link_id.clone()),
            _ => None,
        }
    }
}

impl From<OrderId> for ActiveOrderId {
    fn from(order_id: OrderId) -> Self {
        ActiveOrderId::OrderId(order_id)
    }
}

impl From<OrderLinkId> for ActiveOrderId {
    fn from(order_link_id: OrderLinkId) -> Self {
        ActiveOrderId::OrderLinkId(order_link_id)
    }
}

#[derive(Debug, Default)]
pub struct PlaceActiveOrderData {
    pub symbol: String,
    pub side: Side,
    pub qty: f64,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub close_on_trigger: Option<bool>,
    pub order_link_id: Option<OrderLinkId>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<TriggerPrice>,
    pub sl_trigger_by: Option<TriggerPrice>,
    // linear active specifics
    pub reduce_only: Option<bool>,
    pub position_idx: Option<i64>,
    // conditional specifics
    pub base_price: Option<String>,
    pub stop_px: Option<String>,
    pub trigger_by: Option<TriggerPrice>,
}

#[derive(Debug, Default)]
pub struct PlaceLinearConditionalOrderData {
    pub symbol: String,
    pub side: Side,
    pub qty: f64,
    pub order_type: OrderType,
    pub price: Option<f64>,
    pub time_in_force: TimeInForce,
    pub close_on_trigger: bool,
    pub order_link_id: Option<OrderLinkId>,
    pub take_profit: Option<f64>,
    pub stop_loss: Option<f64>,
    pub tp_trigger_by: Option<TriggerPrice>,
    pub sl_trigger_by: Option<TriggerPrice>,
    // conditional specifics
    pub base_price: f64,
    pub stop_px: f64,
    pub trigger_by: Option<TriggerPrice>,
    // linear conditional specifics
    pub reduce_only: bool,
    pub position_idx: Option<i64>,
}

pub struct UpdateOrderData {
    pub symbol: String,
    pub price: f64,
}

#[async_trait]
pub trait ListActiveOrders {
    async fn list_orders(&self, filter: ListActiveOrdersFilter) -> Result<Vec<Order>>;
}

#[async_trait]
pub trait PlaceActiveOrder {
    async fn place_active_order(&self, data: PlaceActiveOrderData) -> Result<Order>;
}

#[async_trait]
pub trait PlaceActiveLinearOrder {
    async fn place_active_linear_order(&self, data: PlaceActiveOrderData) -> Result<LinearOrder>;
}

#[async_trait]
pub trait UpdateOrders {
    async fn update_order(
        &self,
        active_order_id: ActiveOrderId,
        data: UpdateOrderData,
    ) -> Result<OrderId>;
}

#[async_trait]
pub trait CancelOrders {
    async fn cancel_active_order(&self, active_order_id: ActiveOrderId, symbol: &str)
        -> Result<()>;
    async fn cancel_all_active_orders(&self, symbol: &str) -> Result<Vec<OrderId>>;
}

#[async_trait]
pub trait QueryActiveOrder {
    async fn query_active_order(
        &self,
        active_order_id: ActiveOrderId,
        symbol: &str,
    ) -> Result<Option<Order>>;
}

#[async_trait]
pub trait PlaceConditionalOrder {
    async fn place_conditional_order(&self, data: PlaceActiveOrderData)
        -> Result<ConditionalOrder>;
}

#[async_trait]
pub trait PlaceLinearConditionalOrder {
    async fn place_linear_conditional_order(
        &self,
        data: PlaceLinearConditionalOrderData,
    ) -> Result<ConditionalOrder>;
}

#[async_trait]
impl ListActiveOrders for Client {
    async fn list_orders(&self, filter: ListActiveOrdersFilter) -> Result<Vec<Order>> {
        let query = request::ListActiveOrders { filter };
        let query = self.sign_query(query);
        let response: Response<response::ListActiveOrders> =
            self.get("/v2/private/order/list", &query).await?;
        response.result().map(|res| res.orders)
    }
}

#[async_trait]
impl PlaceActiveOrder for Client {
    async fn place_active_order(&self, data: PlaceActiveOrderData) -> Result<Order> {
        let query: request::CreateOrder = data.into();
        let query = self.sign_query(query);
        let response: Response<Order> = self.post("/v2/private/order/create", &query).await?;
        response.result()
    }
}

#[async_trait]
impl PlaceActiveLinearOrder for Client {
    async fn place_active_linear_order(&self, data: PlaceActiveOrderData) -> Result<LinearOrder> {
        let query: request::CreateOrder = data.into();
        let query = self.sign_query(query);
        let response: Response<LinearOrder> =
            self.post("/private/linear/order/create", &query).await?;
        response.result()
    }
}

#[async_trait]
impl UpdateOrders for Client {
    async fn update_order(
        &self,
        active_order_id: ActiveOrderId,
        data: UpdateOrderData,
    ) -> Result<OrderId> {
        let query = request::UpdateOrder {
            active_order_id,
            symbol: data.symbol.clone(),
            price: data.price,
        };
        let query = self.sign_query(query);
        let response: Response<response::UpdateOrder> =
            self.post("/v2/private/order/replace", &query).await?;
        response.result().map(|res| res.order_id)
    }
}

#[async_trait]
impl CancelOrders for Client {
    async fn cancel_active_order(
        &self,
        active_order_id: ActiveOrderId,
        symbol: &str,
    ) -> Result<()> {
        let query = request::CancelOrder {
            active_order_id,
            symbol: symbol.to_owned(),
        };
        let query = self.sign_query(query);
        let _response: Response<response::CancelOrder> =
            self.post("/v2/private/order/cancel", &query).await?;
        Ok(())
    }

    async fn cancel_all_active_orders(&self, symbol: &str) -> Result<Vec<OrderId>> {
        let query = request::CancelAllOrders {
            symbol: symbol.to_owned(),
        };
        let query = self.sign_query(query);
        let response: Response<response::CancelAllOrders> =
            self.post("/v2/private/order/cancelAll", &query).await?;
        response.result().map(|res| {
            res.orders
                .iter()
                .map(|order| order.cl_ord_id.clone())
                .collect()
        })
    }
}

#[async_trait]
impl QueryActiveOrder for Client {
    async fn query_active_order(
        &self,
        active_order_id: ActiveOrderId,
        symbol: &str,
    ) -> Result<Option<Order>> {
        let query = request::QueryActiveOrder {
            active_order_id,
            symbol: symbol.to_owned(),
        };
        let query = self.sign_query(query);
        let response: Response<Order> = self.get("/v2/private/order", &query).await?;
        Ok(response.result)
    }
}

#[async_trait]
impl PlaceConditionalOrder for Client {
    async fn place_conditional_order(
        &self,
        data: PlaceActiveOrderData,
    ) -> Result<ConditionalOrder> {
        let query: request::CreateOrder = data.into();
        let query = self.sign_query(query);
        let response: Response<ConditionalOrder> =
            self.post("/v2/private/stop-order/create", &query).await?;
        response.result()
    }
}

#[async_trait]
impl PlaceLinearConditionalOrder for Client {
    async fn place_linear_conditional_order(
        &self,
        data: PlaceLinearConditionalOrderData,
    ) -> Result<ConditionalOrder> {
        let query: request::CreateLinearConditionalOrder = data.into();
        let query = self.sign_query(query);
        let response: Response<ConditionalOrder> = self
            .post("/private/linear/stop-order/create", &query)
            .await?;
        response.result()
    }
}

mod request {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(transparent)]
    pub struct ListActiveOrders {
        pub filter: ListActiveOrdersFilter,
    }

    #[derive(Serialize)]
    pub struct CreateOrder {
        pub symbol: String,
        pub side: Side,
        pub qty: f64,
        pub order_type: OrderType,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<f64>,
        pub time_in_force: TimeInForce,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub close_on_trigger: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_link_id: Option<OrderLinkId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub take_profit: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_loss: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tp_trigger_by: Option<TriggerPrice>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sl_trigger_by: Option<TriggerPrice>,
        // linear conditional specifics
        #[serde(skip_serializing_if = "Option::is_none")]
        pub reduce_only: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub position_idx: Option<i64>,
        //conditional specifics
        #[serde(skip_serializing_if = "Option::is_none")]
        pub base_price: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_px: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger_by: Option<TriggerPrice>,
    }

    #[derive(Serialize)]
    pub struct UpdateOrder {
        #[serde(flatten)]
        pub active_order_id: ActiveOrderId,
        pub symbol: String,
        #[serde(rename = "p_r_price")]
        pub price: f64,
    }

    #[derive(Serialize)]
    pub struct CancelOrder {
        #[serde(flatten)]
        pub active_order_id: ActiveOrderId,
        pub symbol: String,
    }

    #[derive(Serialize)]
    pub struct CancelAllOrders {
        pub symbol: String,
    }

    #[derive(Serialize)]
    pub struct QueryActiveOrder {
        #[serde(flatten)]
        pub active_order_id: ActiveOrderId,
        pub symbol: String,
    }

    #[derive(Serialize)]
    pub struct CreateLinearConditionalOrder {
        pub symbol: String,
        pub side: Side,
        pub qty: f64,
        pub order_type: OrderType,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub price: Option<f64>,
        pub time_in_force: TimeInForce,
        pub close_on_trigger: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub order_link_id: Option<OrderLinkId>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub take_profit: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub stop_loss: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tp_trigger_by: Option<TriggerPrice>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sl_trigger_by: Option<TriggerPrice>,
        pub reduce_only: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub position_idx: Option<i64>,
        //conditional specifics
        pub base_price: f64,
        pub stop_px: f64,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub trigger_by: Option<TriggerPrice>,
    }

    impl Query for ListActiveOrders {}
    impl Query for CreateOrder {}
    impl Query for UpdateOrder {}
    impl Query for CancelOrder {}
    impl Query for CancelAllOrders {}
    impl Query for QueryActiveOrder {}
    impl Query for CreateLinearConditionalOrder {}

    impl From<PlaceActiveOrderData> for CreateOrder {
        fn from(data: PlaceActiveOrderData) -> Self {
            CreateOrder {
                symbol: data.symbol,
                side: data.side,
                qty: data.qty,
                order_type: data.order_type,
                price: data.price,
                time_in_force: data.time_in_force,
                close_on_trigger: data.close_on_trigger,
                order_link_id: data.order_link_id,
                take_profit: data.take_profit,
                stop_loss: data.stop_loss,
                tp_trigger_by: data.tp_trigger_by,
                sl_trigger_by: data.sl_trigger_by,
                reduce_only: data.reduce_only,
                position_idx: data.position_idx,
                base_price: data.base_price,
                stop_px: data.stop_px,
                trigger_by: data.trigger_by,
            }
        }
    }

    impl From<PlaceLinearConditionalOrderData> for CreateLinearConditionalOrder {
        fn from(data: PlaceLinearConditionalOrderData) -> Self {
            CreateLinearConditionalOrder {
                symbol: data.symbol,
                side: data.side,
                qty: data.qty,
                order_type: data.order_type,
                price: data.price,
                time_in_force: data.time_in_force,
                close_on_trigger: data.close_on_trigger,
                order_link_id: data.order_link_id,
                take_profit: data.take_profit,
                stop_loss: data.stop_loss,
                tp_trigger_by: data.tp_trigger_by,
                sl_trigger_by: data.sl_trigger_by,
                reduce_only: data.reduce_only,
                position_idx: data.position_idx,
                base_price: data.base_price,
                stop_px: data.stop_px,
                trigger_by: data.trigger_by,
            }
        }
    }
}

mod response {
    use super::{Order, OrderId};
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct ListActiveOrders {
        #[serde(rename = "data")]
        pub orders: Vec<Order>,
    }

    #[derive(Deserialize)]
    pub struct UpdateOrder {
        pub order_id: OrderId,
    }

    #[derive(Deserialize)]
    pub struct CancelOrder {
        pub order_id: OrderId,
    }

    #[derive(Deserialize)]
    #[serde(transparent)]
    pub struct CancelAllOrders {
        pub orders: Vec<CancelledOrder>,
    }

    #[derive(Deserialize)]
    pub struct CancelledOrder {
        #[serde(rename = "clOrdID")]
        pub cl_ord_id: OrderId,
    }
}
