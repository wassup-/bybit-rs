use crate::{
    http::{Client, Query, Response, Result},
    order::*,
};
use async_trait::async_trait;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub struct ListOrdersFilter {
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

pub struct CreateOrderData {
    pub symbol: String,
    pub side: Side,
    pub qty: i64,
    pub price: f64,
    pub order_type: OrderType,
    pub time_in_force: TimeInForce,
}

pub struct UpdateOrderData {
    pub symbol: String,
    pub price: f64,
}

#[async_trait]
pub trait ListOrders {
    async fn list_orders(&self, filter: ListOrdersFilter) -> Result<Vec<Order>>;
}

#[async_trait]
pub trait CreateOrders {
    async fn create_order(&self, data: CreateOrderData) -> Result<Order>;
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
    async fn cancel_order(&self, active_order_id: ActiveOrderId, symbol: &str) -> Result<()>;
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
impl ListOrders for Client {
    async fn list_orders(&self, filter: ListOrdersFilter) -> Result<Vec<Order>> {
        let query = request::ListOrders { filter };
        let query = self.sign_query(query);
        let response: Response<response::ListOrders> =
            self.get("/v2/private/order/list", &query).await?;
        response.result().map(|res| res.orders)
    }
}

#[async_trait]
impl CreateOrders for Client {
    async fn create_order(&self, data: CreateOrderData) -> Result<Order> {
        let query = request::CreateOrder {
            symbol: data.symbol,
            side: data.side,
            qty: data.qty,
            price: data.price,
            order_type: data.order_type,
            time_in_force: data.time_in_force,
        };
        let query = self.sign_query(query);
        let response: Response<Order> = self.post("/v2/private/order/create", &query).await?;
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
    async fn cancel_order(&self, active_order_id: ActiveOrderId, symbol: &str) -> Result<()> {
        let query = request::CancelOrder {
            active_order_id,
            symbol: symbol.to_owned(),
        };
        let query = self.sign_query(query);
        let _response: Response<response::CancelOrder> =
            self.post("/v2/private/order/cancel", &query).await?;
        Ok(())
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

mod request {
    use super::{ActiveOrderId, ListOrdersFilter, OrderType, Query, Side, TimeInForce};
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(transparent)]
    pub struct ListOrders {
        pub filter: ListOrdersFilter,
    }

    #[derive(Serialize)]
    pub struct CreateOrder {
        pub symbol: String,
        pub side: Side,
        pub qty: i64,
        pub price: f64,
        pub order_type: OrderType,
        pub time_in_force: TimeInForce,
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
    pub struct QueryActiveOrder {
        #[serde(flatten)]
        pub active_order_id: ActiveOrderId,
        pub symbol: String,
    }

    impl Query for ListOrders {}
    impl Query for CreateOrder {}
    impl Query for UpdateOrder {}
    impl Query for CancelOrder {}
    impl Query for QueryActiveOrder {}
}

mod response {
    use super::{Order, OrderId};
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct ListOrders {
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
}
