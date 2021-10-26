use bybit::{ws, Result};
use futures_util::StreamExt;

const SYMBOL: &str = "BTCUSD";

#[tokio::main]
async fn main() -> Result<()> {
    println!("printing the next 5 order book updates for {}", SYMBOL);

    let mut client = ws::Client::new(ws::MAINNET_BYBIT, "", "");
    client.connect().await?;
    client
        .subscribe(&[ws::Channel::OrderBook25(SYMBOL.to_owned())])
        .await?;

    for _ in 0..5 {
        match client.next().await {
            Some(Ok(data)) => println!("{:?}", data),
            _ => (),
        }
    }

    client.disconnect().await?;

    Ok(())
}
