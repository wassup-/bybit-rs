use bybit::{http, rest::*, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3 {
        println!("usage: cargo run --example http_wallet_balances api_key api_secret");
        return Ok(());
    }

    let api_key = &args[1];
    let api_secret = &args[2];

    println!("printing wallet balances");

    // safe to unwrap because we know url is valid
    let client = http::Client::new(http::MAINNET_BYBIT, api_key, api_secret).unwrap();
    let wallets = client.fetch_wallets().await?;
    for currency in wallets.currencies() {
        let wallet = wallets.get(&currency).unwrap();
        println!("{}: {}", currency, wallet.wallet_balance);
    }

    Ok(())
}
