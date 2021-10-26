use bybit::{ws, Result};
use futures_util::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    println!("printing the next 5 trades");

    let mut client = ws::Client::new(ws::MAINNET_BYBIT, "", "");
    client.connect().await?;
    client.subscribe(&[ws::Channel::Trade]).await?;

    for _ in 0..5 {
        match client.next().await {
            Some(Ok(ws::Data::Trade(trade))) => println!("{:?}", trade),
            _ => (),
        }
    }

    client.disconnect().await?;

    Ok(())
}
