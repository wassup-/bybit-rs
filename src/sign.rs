use hmac_sha256::HMAC;
use serde::Serialize;

pub struct Signed<T> {
    pub payload: T,
    pub signature: String,
}

impl<T: Serialize + Sized> Signed<T> {
    pub fn new(payload: T, secret: &str) -> Self {
        let signature = Signed::sign(&payload, secret);
        Self { payload, signature }
    }

    pub fn sign(payload: &T, secret: &str) -> String {
        let query = serde_urlencoded::to_string(payload).unwrap();
        let mut params: Vec<String> = query.split('&').map(|s| s.to_owned()).collect();
        params.sort();

        let query = params.join("&");
        let mac = HMAC::mac(query.as_bytes(), secret.as_bytes());
        hex::encode(mac)
    }
}

#[cfg(test)]
mod tests {
    use super::Signed;
    use serde::Serialize;

    #[derive(Serialize, Eq, PartialEq, Debug, Clone)]
    struct TestPayload<'a> {
        symbol: &'a str,
        timestamp: i64,
        api_key: &'a str,
    }

    #[derive(Serialize)]
    struct NoPayload;

    #[test]
    fn sign_payload() {
        let payload = TestPayload {
            symbol: "BTCUSD",
            timestamp: 1542434791000,
            api_key: "c42a328f503efa30e940712d1131eb14",
        };
        let signature = Signed::sign(&payload, "e280931f830719d1141eb14b42a328f530ab3c16a2");
        assert_eq!(
            signature,
            "13594a73bbc0c01ffbdb796c3a06e9820f31344a4cac2c54624aa8220ec49fdb"
        )
    }

    #[test]
    fn sign_nothing() {
        let payload = NoPayload;
        let signature = Signed::sign(&payload, "e280931f830719d1141eb14b42a328f530ab3c16a2");
        assert_eq!(
            signature,
            "bd1df63a4addbf80ac6cfc339266a355fa88ed7ada7dcbb846d2c46e53b2524c"
        )
    }

    #[test]
    fn signed_payload() {
        let payload = TestPayload {
            symbol: "ETHUSD",
            timestamp: 1628418957000,
            api_key: "8c0020c1e0ac401188f5540037841092",
        };
        let signed = Signed::new(
            payload.clone(),
            "c9fb33e1729346bf83886a4c972dbbb777c84d82b4",
        );
        assert_eq!(signed.payload, payload);
        assert_eq!(
            signed.signature,
            "95f0502697233cfa1681d030158eeb442ea6e385ef74f2b575c65dc6f39e5084"
        );
    }
}
