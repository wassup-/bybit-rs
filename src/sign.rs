use hmac_sha256::HMAC;

#[inline]
pub fn sign(payload: &str, secret: &str) -> String {
    let mac = HMAC::mac(payload.as_bytes(), secret.as_bytes());
    hex::encode(mac)
}

#[cfg(test)]
mod tests {
    #[test]
    fn sign_nothing() {
        let signature = super::sign("", "e280931f830719d1141eb14b42a328f530ab3c16a2");
        assert_eq!(
            signature,
            "bd1df63a4addbf80ac6cfc339266a355fa88ed7ada7dcbb846d2c46e53b2524c"
        )
    }

    #[test]
    fn sign_something() {
        let signature = super::sign("{}", "a280931f830719d1141eb14b42a358f530ab3c16c2");
        assert_eq!(
            signature,
            "01a16d7fa0f0bf4ff0eef202260611b4408e56b9e0c43a8d1c68884152954d43"
        )
    }
}
