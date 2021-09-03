pub fn sign(expires: i64, secret: &str) -> String {
    let payload = format!("GET/realtime{}", expires);
    crate::sign(&payload, secret)
}
