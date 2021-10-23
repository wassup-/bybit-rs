use crate::{
    announcement::*,
    http::{Client, NoQuery, Response, Result},
};
use async_trait::async_trait;

#[async_trait]
pub trait Announcements {
    async fn announcements(&self) -> Result<Vec<Announcement>>;
}

#[async_trait]
impl Announcements for Client {
    async fn announcements(&self) -> Result<Vec<Announcement>> {
        let query = NoQuery::new();
        let response: Response<response::Announcements> =
            self.get("/v2/public/announcement", &query).await?;
        response.result().map(|res| res.announcements)
    }
}

mod response {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(transparent)]
    pub struct Announcements {
        pub announcements: Vec<Announcement>,
    }
}
