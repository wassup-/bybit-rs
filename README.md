[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](CODE_OF_CONDUCT.md)

# Bybit-rs

Unofficial API connector for the [Bybit API](https://bybit-exchange.github.io/docs/inverse).

## Disclaimer

This software is for educational purposes only. Do not risk money which you are afraid to lose.
USE THE SOFTWARE AT YOUR OWN RISK. THE AUTHORS AND ALL AFFILIATES ASSUME NO RESPONSIBILITY FOR YOUR TRADING RESULTS.

## Installation

Add the following dependency to your Cargo.toml
```yaml
[dependencies.bybit]
git = "https://github.com/wassup-/bybit-rs.git"
```

## Usage

```rs
use bybit::rest::{self, *};
use bybit::{http::Client, Order, OrderType, Result, Side, TimeInForce};

async fn create_order() -> Result<Order> {
    let client = Client::new(api_key, api_secret);
    let data = rest::CreateOrderData {
        symbol: "BTCUSD",
        side: Side::Sell,
        qty: 10,
        price: 45420.0,
        order_type: OrderType::Limit,
        time_in_force: TimeInForce::PostOnly,
    };
    client.create_order(data).await
}
```

## Status

bybit-rs is still under active development. This means that until this library reaches version 1.0.0, things are prone to break and/or change.

## Contributing

To get involved, take a look at [CONTRIBUTING](CONTRIBUTING.md).

## License

This library is provided under the MIT license. See [LICENSE](LICENSE).

## Support the author of this library.

If bybit-rs made your life easier, please consider making a donation.

- BTC `1NCLMTd4Zh6hcWxnVk9emfgyyRabEACy1m`
- ETH `0xab3e024d41b6e9eb6b03bd56de9fdf077c904ef9`
- EOS `bybitdeposit` tag/memo: `3493784`
- XRP `rJn2zAPdFA193sixJwuFixRkYDUtx3apQh` tag/memo: `3493784`
