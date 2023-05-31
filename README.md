<p align="center">
<img src="https://app.freecurrencyapi.com/img/logo/freecurrencyapi.png" width="300"/>
</p>

# freecurrencyapi-rs: Rust Currency Converter

This package is a Rust wrapper for [freecurrencyapi.com](https://freecurrencyapi.com) that aims to make the usage of the API as easy as possible in your project.
Freecurrencyapi.com is a free currency API that provides realtime as well as historic foreign exchange data.


## Installation

This crate is under development. Especially the response parsing needs some more testing. However, if you still want to use it, you can install it by adding this to your `Cargo.toml`:

```toml
[dependencies]
freecurrencyapi = "0.1.0"
```

## Requirements

1. API Key for [freecurrencyapi.com](https://freecurrencyapi.com/)
2. Async runtime like [tokio](https://crates.io/crates/tokio)

## Quickstart

```rust
use freecurrencyapi::Freecurrencyapi;
use freecurrencyapi::models;

async fn request_latest() -> Result<models::DetailsResponse, freecurrencyapi::Error> {
    let c_api = Freecurrencyapi::new("<your-api-key>")?;
    let details = c_api.latest("id-of-a-fuel-station").await?;
     Ok(details)
}
```

Endpoints accessible with a free account:
- `status`
- `currencies`
- `latest`
- `historical`

Find out more about our endpoints, parameters and response data structure in the [docs](https://freecurrencyapi.com/docs)

## License

The MIT License (MIT). Please see [License File](LICENSE.md) for more information.

[docs]: https://freecurrencyapi.com/docs
[freecurrencyapi.com]: https://freecurrencyapi.com