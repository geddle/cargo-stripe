use anyhow::{Result, anyhow};
use std::collections::HashSet;
use std::str;

/// List of supported components
pub fn supported_components() -> HashSet<&'static str> {
    let mut components = HashSet::new();
    components.insert("customer");
    components.insert("charge");
    components.insert("payment_intent");
    components.insert("payment_method");
    components.insert("refund");
    components.insert("product");
    components.insert("price");
    components.insert("subscription");
    components.insert("invoice");
    components.insert("checkout");
    components.insert("webhook");
    // Add more components as needed
    components
}

/// Check if a component is valid
pub fn is_valid_component(component: &str) -> bool {
    supported_components().contains(component)
}

/// Generate the content for a specific component
pub fn generate_component(component: &str) -> Result<String> {
    match component {
        "customer" => {
            Ok(str::from_utf8(include_bytes!("templates/components/customer.rs"))?.to_string())
        }
        "charge" => {
            Ok(str::from_utf8(include_bytes!("templates/components/charge.rs"))?.to_string())
        }
        "payment_intent" => Ok(str::from_utf8(include_bytes!(
            "templates/components/payment_intent.rs"
        ))?
        .to_string()),
        // "payment_method" => Ok(str::from_utf8(include_bytes!(
        //     "templates/components/payment_method.rs"
        // ))?
        // .to_string()),
        "refund" => {
            Ok(str::from_utf8(include_bytes!("templates/components/refund.rs"))?.to_string())
        }
        "product" => {
            Ok(str::from_utf8(include_bytes!("templates/components/product.rs"))?.to_string())
        }
        "price" => Ok(str::from_utf8(include_bytes!("templates/components/price.rs"))?.to_string()),
        "subscription" => {
            Ok(str::from_utf8(include_bytes!("templates/components/subscription.rs"))?.to_string())
        }
        "invoice" => {
            Ok(str::from_utf8(include_bytes!("templates/components/invoice.rs"))?.to_string())
        }
        // "checkout" => {
        //     Ok(str::from_utf8(include_bytes!("templates/components/checkout.rs"))?.to_string())
        // }
        // "webhook" => {
        //     Ok(str::from_utf8(include_bytes!("templates/components/webhook.rs"))?.to_string())
        // }
        _ => Err(anyhow!("Unsupported component: {}", component)),
    }
}