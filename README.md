# cargo-stripe

A cargo extension for adding Stripe API components to your Rust projects.

## Installation

```bash
cargo install cargo-stripe
```

## Usage

### Initialize the base Stripe SDK

```bash
cargo stripe init
```

This command sets up the basic Stripe SDK structure in your project's `src/stripe` directory. It creates the following files:

- `mod.rs`: Main module file that exports all components
- `client.rs`: The Stripe API client with authentication
- `error.rs`: Error types and handling
- `types.rs`: Common types used across components

### Add a specific API component

```bash
cargo stripe add customer
```

This command adds a specific Stripe API component to your project. For example, `cargo stripe add customer` will add the Customer API in `src/stripe/customer.rs`.

Available components:
- `customer`: Customer API
- `charge`: Charge API
- `payment_intent`: Payment Intent API
- `payment_method`: Payment Method API
- `refund`: Refund API
- `product`: Product API
- `price`: Price API
- `subscription`: Subscription API
- `invoice`: Invoice API
- `checkout`: Checkout API
- `webhook`: Webhook handling

## Using the SDK

After initializing the SDK and adding the components you need, you can use them in your code like this:

```rust
use stripe::{Client, CreateCustomer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new client with your API key
    let client = Client::new("sk_test_your_api_key")?;
    
    // Create a new customer
    let customer_params = CreateCustomer {
        email: Some("customer@example.com".to_string()),
        name: Some("John Doe".to_string()),
        ..Default::default()
    };
    
    let customer = client.create_customer(&customer_params).await?;
    println!("Created customer: {:?}", customer);
    
    Ok(())
}
```

## Features

- Modular, component-based approach - only include what you need
- Idiomatic Rust API design
- Type-safe request and response handling
- Comprehensive error handling
- Asynchronous API using tokio

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

MIT License