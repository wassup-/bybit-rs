use bybit::{http, rest::*, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("usage: cargo run --example http_asset_exchange_records api_key api_secret");
        return Ok(());
    }

    let api_key = &args[1];
    let api_secret = &args[2];

    println!("printing 10 asset exchange records");

    // safe to unwrap because we know url is valid
    let client = http::Client::new(http::MAINNET_BYBIT, api_key, api_secret).unwrap();
    let options = FetchAssetExchangeRecordsOptions {
        limit: Some(10),
        ..Default::default()
    };
    let records = client.fetch_asset_exchange_records(options).await?;
    for record in records.iter() {
        println!("{:?}", record);
    }

    Ok(())
}
