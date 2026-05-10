# facture_rs

[![Crates.io](https://img.shields.io/crates/v/facture_rs)](https://crates.io/crates/facture_rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-2024edition-orange.svg)](https://www.rust-lang.org)

A Rust wrapper for the [FactureApp API](https://api.facture.com.mx), focused on Mexican electronic invoicing (CFDI) and billing operations.

## Introduction

`facture_rs` provides a type-safe, asynchronous Rust interface to interact with the FactureApp API. It simplifies authentication, electronic stamping (timbrado), branch management, client administration, and other billing-related operations required for CFDI compliance in Mexico.

## Features

- **Authentication**: Password-based OAuth2, refresh tokens, and authorization code flows
- **Electronic Stamping (Timbrado)**: Stamp CFDI invoices in JSON format
- **Branch Offices (Sucursales)**: Query and manage branch office information
- **Clients**: Client creation and management
- **Concepts**: Product/service concept management
- **Self-Invoicing**: Self-billing operations
- **Cancellation**: Invoice cancellation workflows
- **Invoice Management**: Query and manage issued invoices
- **Emisor**: Taxpayer (emisor) information management

## Installation

Add `facture_rs` to your `Cargo.toml`:

```toml
[dependencies]
facture_rs = "0.1.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Or use cargo add:

```bash
cargo add facture_rs
cargo add tokio --features macros,rt-multi-thread
```

## Usage

### Authentication

Obtain an access token using password credentials:

```rust
use facture_rs::{
    auth::{password::Password, utils::input::PasswordInput},
    core::{app::App, credentials::AppCredentials, request::FactureRequest}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let scopes = "timbrado sucursal facturacion cancelacion".to_string();

    let credentials = AppCredentials::new()
        .user("your_email@example.com")
        .password("your_password");

    let app = App::new()
        .set_client_id("your_client_id")
        .set_client_secret("your_client_secret")
        .set_credentials(credentials);

    let input = PasswordInput {
        app,
        scopes
    };

    let response = Password.request(input).await?;

    println!("Access Token: {}", response.access_token);
    println!("Refresh Token: {}", response.refresh_token);
    println!("Expires In: {} seconds", response.expires_in);

    Ok(())
}
```

### Electronic Stamping (Timbrado)

Stamp a CFDI invoice using a JSON payload:

```rust
use std::fs;
use base64::{Engine, engine::general_purpose};
use facture_rs::{
    belling::{json::JsonBilling, utils::{core::comprobante::Comprobante, json::input::JsonInput}},
    core::request::FactureRequest
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bearer = "your_access_token".to_string();

    // Read your CFDI JSON payload
    let tax = fs::read_to_string("examples/simple_tax.json")
        .expect("Failed to read CFDI file");

    // Base64 encode the payload
    let encode = general_purpose::STANDARD.encode(tax);

    let comprobante = Comprobante {
        encode,
        request_uuid: "123e4567-e89b-12d3-a456-426655440000".to_string()
    };

    let input = JsonInput::new()
        .set_bearer(bearer)
        .set_comprobante(comprobante)
        .set_emisor(61400)
        .set_sucursal(63073);

    let response = JsonBilling.request(input).await?;

    println!("Response Code: {}", response.code);
    println!("Message: {}", response.message);

    for item in response.result.items {
        println!("Succeed: {}", item.succeed);
        println!("UUID: {}", item.uuid);
        println!("Message: {}", item.message);
    }

    Ok(())
}
```

### Branch Offices (Sucursales)

Query branch offices with filters:

```rust
use facture_rs::{
    core::request::FactureRequest,
    sucursal::{get::Sucursal, utils::{filters::SucursalFilters, input::SucursalInput}}
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bearer = "your_access_token".to_string();

    let filters = SucursalFilters::new()
        .offset(0)
        .size(10)
        .emisor("EKU9003173C9");

    let input = SucursalInput {
        bearer,
        filters
    };

    let response = Sucursal.request(input).await?;

    for item in response.pagination.items {
        println!("Branch: {} - {}", item.id, item.nombre);
    }

    Ok(())
}
```

## How to Compile

This library requires **Rust 1.85+** (2024 edition).

### Development Build

```bash
cargo build
```

### Release Build

```bash
cargo build --release
```

### Running Tests

```bash
# Run all tests
cargo test

# Run a specific test
cargo test password_auth
cargo test billing_json

# Run tests with output
cargo test -- --nocapture
```

### Examples

The `examples/` directory contains sample CFDI JSON payloads:

```bash
# Example CFDI structure
examples/simple_tax.json
```

## API Overview

| Module | Description | Status |
|--------|-------------|--------|
| `auth` | OAuth2 authentication flows | Implemented |
| `belling` | Electronic stamping (timbrado) | Implemented |
| `sucursal` | Branch office management | Implemented |
| `client` | Client management | Implemented |
| `concept` | Product/service concepts | Implemented |
| `self_invoice` | Self-billing operations | Implemented |
| `cancelation` | Invoice cancellation | Implemented |
| `invoicement` | Invoice management | Implemented |
| `emisor` | Taxpayer information | Implemented |

## API Base URL

The library communicates with:

```
https://app.facture.com.mx/api
```

## Dependencies

- [reqwest](https://crates.io/crates/reqwest) — Async HTTP client
- [serde](https://crates.io/crates/serde) — Serialization framework
- [serde_json](https://crates.io/crates/serde_json) — JSON support
- [tokio](https://crates.io/crates/tokio) — Async runtime
- [thiserror](https://crates.io/crates/thiserror) — Error handling
- [base64](https://crates.io/crates/base64) — Base64 encoding

## Error Handling

The library uses a custom error type per module, all implementing `std::error::Error`:

- `AuthError` — Authentication errors (network, parse, API)
- `BillingError` — Stamping errors (network, parse, API, setup)
- `SucursalError` — Branch office query errors
- And more...

Each error provides detailed context about what went wrong.

## License

This project is licensed under the **MIT License**.

```
MIT License

Copyright (c) 2024 facture_rs contributors

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

## Disclaimer

This is an unofficial wrapper. Please refer to the [official FactureApp documentation](https://api.facture.com.mx) for complete API specifications and usage guidelines.

## See Also

- [FactureApp](https://facture.com.mx) — Official website
- [CFDI Documentation](https://www.sat.gob.mx) — Mexican Tax Authority (SAT)
- [Rust Async Book](https://rust-lang.github.io/async-book/) — Learn async Rust
