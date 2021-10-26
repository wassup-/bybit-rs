use bybit::{http, rest::*, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("usage: cargo run --example http_wallet_fund_records api_key api_secret");
        return Ok(());
    }

    let api_key = &args[1];
    let api_secret = &args[2];

    // safe to unwrap because we know url is valid
    let client = http::Client::new(http::MAINNET_BYBIT, api_key, api_secret).unwrap();
    fund_records(&client).await?;

    Ok(())
}

async fn fund_records(client: &http::Client) -> Result<()> {
    println!("printing 10 latest wallet fund records");

    let options = FetchWalletFundRecordsOptions {
        limit: Some(10),
        ..Default::default()
    };
    let records = client.fetch_wallet_fund_records(options).await?;
    for record in records.iter() {
        println!("{:?}", record);
    }

    Ok(())
}
