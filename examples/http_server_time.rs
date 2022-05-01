use bybit::{http, rest::*, Result};
use chrono::Utc;

#[tokio::main]
async fn main() -> Result<()> {
    println!("printing the server time");

    // safe to unwrap because we know url is valid
    let client = http::Client::new(http::MAINNET_BYBIT, "", "").unwrap();
    let server_time = client.server_time().await?;
    let local_time = Utc::now();

    println!("server time: {}", server_time);
    println!(
        "local time : {}.{}",
        local_time.timestamp(),
        local_time.timestamp_subsec_micros()
    );

    Ok(())
}
