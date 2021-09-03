use super::{Query, Response, Result, SignedQuery};
use reqwest::Url;
use serde::{de::DeserializeOwned, Serialize};

pub const MAINNET_BYBIT: &str = "https://api.bybit.com/";
pub const MAINNET_BYTICK: &str = "https://api.bytick.com/";
pub const TESTNET: &str = "https://api-testnet.bybit.com/";

pub struct Client {
    api_key: String,
    api_secret: String,
    client: reqwest::Client,
    base_url: Url,
}

impl Client {
    /// Create a new client.
    /// * `base_url` - The base url.
    /// * `api_key` - The API key.
    /// * `api_secret` - The API secret.
    pub fn new(base_url: &str, api_key: &str, api_secret: &str) -> Self {
        Client {
            api_key: api_key.to_owned(),
            api_secret: api_secret.to_owned(),
            client: reqwest::Client::new(),
            base_url: Url::parse(base_url).unwrap(),
        }
    }

    /// Sign a query.
    pub fn sign_query<Q: Query>(&self, query: Q) -> SignedQuery<Q> {
        SignedQuery::sign(query, self.api_key.as_str(), self.api_secret.as_str())
    }

    /// Perform a GET request and return the response.
    /// * `path` - The path of the URL to request.
    /// * `query` - The query to send with the request.
    pub async fn get<Q: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<Response<T>> {
        let url = self.base_url.join(path).unwrap();
        let response = self.client.get(url).query(query).send().await?;
        let result = response.json::<Response<T>>().await?;
        Ok(result)
    }

    /// Perform a POST request and return the response.
    /// * `path` - The path of the URL to request.
    /// * `query` - The query to send with the request.
    pub async fn post<Q: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<Response<T>> {
        let url = self.base_url.join(path).unwrap();
        let response = self.client.post(url).json(query).send().await?;
        let result = response.json::<Response<T>>().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let client = Client::new(MAINNET_BYBIT, "key", "secret");
        assert_eq!(client.api_key, "key");
        assert_eq!(client.api_secret, "secret");
        assert_eq!(client.base_url.host_str(), Some("api.bybit.com"));
    }

    #[test]
    fn test() {
        let client = Client::new(TESTNET, "key", "secret");
        assert_eq!(client.api_key, "key");
        assert_eq!(client.api_secret, "secret");
        assert_eq!(client.base_url.host_str(), Some("api-testnet.bybit.com"));
    }
}
