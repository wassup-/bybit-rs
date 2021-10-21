use crate::{
    http::{Client, Response, Result},
    LiquidatedOrder,
};
use async_trait::async_trait;

#[derive(Clone, Debug, Default)]
pub struct LiquidatedOrdersFilter {
    pub symbol: String,
    pub from: i64,
    pub limit: i64,
    pub start_time: i64,
    pub end_time: i64,
}

#[async_trait]
pub trait LiquidatedOrders {
    async fn get_liquidated_orders(
        &mut self,
        filter: LiquidatedOrdersFilter,
    ) -> Result<Vec<LiquidatedOrder>>;
}

#[async_trait]
impl LiquidatedOrders for Client {
    async fn get_liquidated_orders(
        &mut self,
        filter: LiquidatedOrdersFilter,
    ) -> Result<Vec<LiquidatedOrder>> {
        let query = query::LiquidatedOrders {
            symbol: filter.symbol,
            from: filter.from.to_owned(),
            limit: filter.limit.to_owned(),
            start_time: filter.start_time.to_owned(),
            end_time: filter.end_time.to_owned(),
        };
        let response: Response<response::LiquidatedOrders> =
            self.get("/v2/public/liq-records", &query).await?;
        response.result().map(|res| res.data)
    }
}

mod query {
    use serde::Serialize;

    #[derive(Serialize)]
    pub struct LiquidatedOrders {
        pub symbol: String,
        pub from: i64,
        pub limit: i64,
        pub start_time: i64,
        pub end_time: i64,
    }
}

mod response {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(transparent)]
    pub struct LiquidatedOrders {
        pub data: Vec<LiquidatedOrder>,
    }
}
