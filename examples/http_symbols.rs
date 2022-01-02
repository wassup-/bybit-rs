use bybit::{http, rest::*, Result};

#[tokio::main]
async fn main() -> Result<()> {
    println!("printing all symbols");

    // safe to unwrap because we know url is valid
    let client = http::Client::new(http::MAINNET_BYBIT, "", "").unwrap();
    let symbols = client.fetch_symbols().await?;

    for symbol in symbols.symbols() {
        println!("{:?}", symbol);
    }

    Ok(())
}
