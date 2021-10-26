use bybit::{http, rest::*, Result};

const SYMBOL: &str = "BTCUSD";

#[tokio::main]
async fn main() -> Result<()> {
    println!("printing the tickers for {}", SYMBOL);

    // safe to unwrap because we know url is valid
    let client = http::Client::new(http::MAINNET_BYBIT, "", "").unwrap();
    let tickers = client.fetch_tickers(SYMBOL).await?;

    for ticker in tickers.tickers() {
        println!("{:?}", ticker);
    }

    Ok(())
}
