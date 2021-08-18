use crate::Signed;
use chrono::Utc;
use serde::Serialize;
use std::marker::PhantomData;

pub trait Query: Serialize + Sized {}

#[derive(Serialize)]
pub struct NoQuery {
    #[serde(skip)]
    data: PhantomData<()>,
}

#[derive(Serialize, Clone, Eq, PartialEq, Debug)]
pub struct SignedQuery<Q: Query> {
    #[serde(flatten)]
    query: Q,
    timestamp: i64,
    api_key: String,
    #[serde(rename = "sign")]
    signature: String,
}

impl<Q: Query> SignedQuery<Q> {
    /// Create a new signed query.
    pub fn new(query: Q, timestamp: i64, api_key: &str, secret: &str) -> Self {
        let payload = Payload {
            query,
            timestamp,
            api_key: api_key.to_owned(),
        };
        Signed::new(payload, secret).into()
    }

    pub fn sign(query: Q, api_key: &str, secret: &str) -> Self {
        SignedQuery::new(query, Utc::now().timestamp_millis(), api_key, secret)
    }
}

#[derive(Serialize)]
struct Payload<Q: Query> {
    #[serde(flatten)]
    query: Q,
    timestamp: i64,
    api_key: String,
}

impl<Q: Query> From<Signed<Payload<Q>>> for SignedQuery<Q> {
    fn from(signed: Signed<Payload<Q>>) -> Self {
        Self {
            query: signed.payload.query,
            timestamp: signed.payload.timestamp,
            api_key: signed.payload.api_key,
            signature: signed.signature,
        }
    }
}

impl Query for NoQuery {}

impl NoQuery {
    pub fn new() -> Self {
        Self { data: PhantomData }
    }
}

impl Default for NoQuery {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{NoQuery, Query, SignedQuery};
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestQuery<'a> {
        item: &'a str,
    }

    impl<'a> Query for TestQuery<'a> {}

    #[test]
    fn signed_query() {
        let query = TestQuery { item: "test" };
        let signed = SignedQuery::new(query, 1626034022751, "test-api-key", "test-secret");
        assert_eq!(
            signed.signature,
            "943a54d33d58367bf7a16549e12e6ade546da26b70bc9fece8c11eb1f6717cb1"
        );
    }

    #[test]
    fn signed_no_query() {
        let query = NoQuery::new();
        let signed = SignedQuery::new(query, 1626034022751, "test-api-key", "test-secret");
        assert_eq!(
            signed.signature,
            "a87d38f97890c9856e0a88e8ed55b61664792954b7adb3ef4748493379356fa8"
        );
    }
}
