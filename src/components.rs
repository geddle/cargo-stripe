use anyhow::{Result, anyhow};
use std::collections::{HashMap, HashSet};
use std::str;

/// Component file mapping for both extension and generated files
pub struct ComponentFiles {
    pub extension_file: Option<String>,
    pub generated_files: Vec<String>,
}

/// List of supported resource components
pub fn supported_components() -> HashSet<&'static str> {
    let mut components = HashSet::new();
    
    // Core resources
    components.insert("account");
    components.insert("balance");
    components.insert("balance_transaction");
    components.insert("bank_account");
    components.insert("card");
    components.insert("charge");
    components.insert("checkout_session");
    components.insert("credit_note");
    components.insert("currency");
    components.insert("customer");
    components.insert("customer_balance_transaction");
    components.insert("invoice");
    components.insert("issuing_authorization");
    components.insert("issuing_card");
    components.insert("issuing_dispute");
    components.insert("issuing_merchant_data");
    components.insert("issuing_transaction");
    components.insert("line_item");
    components.insert("login_links");
    components.insert("order");
    components.insert("payment_intent");
    components.insert("payment_method");
    components.insert("payment_source");
    components.insert("payout");
    components.insert("price");
    components.insert("product");
    components.insert("promotion_code");
    components.insert("refund");
    components.insert("review");
    components.insert("setup_intent");
    components.insert("source");
    components.insert("subscription");
    components.insert("test_clock");
    components.insert("token");
    components.insert("transfer_reversal");
    components.insert("usage_record");
    components.insert("webhook_endpoint");
    components.insert("webhook_events");
    
    // Add all resources option
    components.insert("all");
    
    components
}

/// Check if a component is valid
pub fn is_valid_component(component: &str) -> bool {
    supported_components().contains(component)
}

/// Get file mapping for a specific component
pub fn get_component_file_mapping(component: &str) -> Result<ComponentFiles> {
    match component {
        "account" => Ok(ComponentFiles {
            extension_file: Some("account_ext.rs".to_string()),
            generated_files: vec![
                "account.rs".to_string(),
                "account_link.rs".to_string(),
                "account_session.rs".to_string(),
                "account_application_authorized.rs".to_string(),
                "account_application_deauthorized.rs".to_string(),
                "account_external_account_created.rs".to_string(),
                "account_external_account_deleted.rs".to_string(),
                "account_external_account_updated.rs".to_string(),
                "account_updated.rs".to_string(),
            ],
        }),
        "balance" => Ok(ComponentFiles {
            extension_file: Some("balance_ext.rs".to_string()),
            generated_files: vec![
                "balance.rs".to_string(),
                "balance_amount_by_source_type.rs".to_string(),
                "balance_available.rs".to_string(),
            ],
        }),
        "balance_transaction" => Ok(ComponentFiles {
            extension_file: Some("balance_transaction_ext.rs".to_string()),
            generated_files: vec![
                "balance_transaction.rs".to_string(),
            ],
        }),
        "bank_account" => Ok(ComponentFiles {
            extension_file: Some("bank_account_ext.rs".to_string()),
            generated_files: vec![
                "bank_account.rs".to_string(),
            ],
        }),
        "card" => Ok(ComponentFiles {
            extension_file: Some("card.rs".to_string()),
            generated_files: vec![
                "card.rs".to_string(),
            ],
        }),
        "charge" => Ok(ComponentFiles {
            extension_file: Some("charge_ext.rs".to_string()),
            generated_files: vec![
                "charge.rs".to_string(),
                "charge_captured.rs".to_string(),
                "charge_expired.rs".to_string(),
                "charge_failed.rs".to_string(),
                "charge_pending.rs".to_string(),
                "charge_refunded.rs".to_string(),
                "charge_succeeded.rs".to_string(),
                "charge_updated.rs".to_string(),
            ],
        }),
        "checkout_session" => Ok(ComponentFiles {
            extension_file: Some("checkout_session_ext.rs".to_string()),
            generated_files: vec![
                "checkout_session.rs".to_string(),
                "checkout_session_async_payment_failed.rs".to_string(),
                "checkout_session_async_payment_succeeded.rs".to_string(),
                "checkout_session_completed.rs".to_string(),
                "checkout_session_expired.rs".to_string(),
            ],
        }),
        "customer" => Ok(ComponentFiles {
            extension_file: Some("customer_ext.rs".to_string()),
            generated_files: vec![
                "customer.rs".to_string(),
                "customer_created.rs".to_string(),
                "customer_deleted.rs".to_string(),
                "customer_updated.rs".to_string(),
                "customer_discount_created.rs".to_string(),
                "customer_discount_deleted.rs".to_string(),
                "customer_discount_updated.rs".to_string(),
                "customer_source_created.rs".to_string(),
                "customer_source_deleted.rs".to_string(),
                "customer_source_expiring.rs".to_string(),
                "customer_source_updated.rs".to_string(),
                "customer_subscription_created.rs".to_string(),
                "customer_subscription_deleted.rs".to_string(),
                "customer_subscription_updated.rs".to_string(),
                "customer_tax_id_created.rs".to_string(),
                "customer_tax_id_deleted.rs".to_string(),
                "customer_tax_id_updated.rs".to_string(),
            ],
        }),
        "payment_intent" => Ok(ComponentFiles {
            extension_file: Some("payment_intent_ext.rs".to_string()),
            generated_files: vec![
                "payment_intent.rs".to_string(),
                "payment_intent_amount_capturable_updated.rs".to_string(),
                "payment_intent_canceled.rs".to_string(),
                "payment_intent_created.rs".to_string(),
                "payment_intent_partially_funded.rs".to_string(),
                "payment_intent_payment_failed.rs".to_string(),
                "payment_intent_processing.rs".to_string(),
                "payment_intent_requires_action.rs".to_string(),
                "payment_intent_succeeded.rs".to_string(),
            ],
        }),
        "payment_method" => Ok(ComponentFiles {
            extension_file: Some("payment_method_ext.rs".to_string()),
            generated_files: vec![
                "payment_method.rs".to_string(),
                "payment_method_attached.rs".to_string(),
                "payment_method_automatically_updated.rs".to_string(),
                "payment_method_detached.rs".to_string(),
                "payment_method_updated.rs".to_string(),
                // Many payment method types files would be listed here...
                "payment_method_card.rs".to_string(),
                "payment_method_sepa_debit.rs".to_string(),
            ],
        }),
        "product" => Ok(ComponentFiles {
            extension_file: Some("product_ext.rs".to_string()),
            generated_files: vec![
                "product.rs".to_string(),
                "product_created.rs".to_string(),
                "product_deleted.rs".to_string(),
                "product_updated.rs".to_string(),
            ],
        }),
        // For brevity, I'm not listing all components with their complete file mappings
        // In a real implementation, you would need to include all components
        
        // Special case for "all"
        "all" => Err(anyhow!("The 'all' component should be handled separately")),
        
        // For any other component, provide a default mapping based on naming convention
        _ => {
            let ext_file = format!("{}_ext.rs", component);
            let base_file = format!("{}.rs", component);
            
            Ok(ComponentFiles {
                extension_file: Some(ext_file),
                generated_files: vec![base_file],
            })
        }
    }
}

/// Generate the content for a specific extension file
pub fn generate_extension_file(component: &str, filename: &str) -> Result<String> {
    // In a real implementation, this would load the actual extension file templates
    Ok(format!(
        "//! Extension methods for the Stripe {} resource\n\nuse crate::stripe::resources::generated::{};\n\n// Extension methods would be defined here\n",
        component, component
    ))
}

/// Generate the content for a specific generated file
pub fn generate_generated_file(filename: &str) -> Result<String> {
    // In a real implementation, this would load the actual generated file templates
    let resource_name = filename.trim_end_matches(".rs").replace('_', " ");
    
    Ok(format!(
        "//! Generated code for Stripe {} resource\n\n// Resource definition would be here\n",
        resource_name
    ))
}

/// Generate the content for resources/types.rs
pub fn generate_resource_types_file() -> Result<String> {
    Ok("//! Common types used in Stripe API resources\n\n// Type definitions would be here\n".to_string())
}

/// Generate the content for resources/generated.rs
pub fn generate_resource_generated_file() -> Result<String> {
    Ok("//! Re-exports all generated resource definitions\n\npub use super::generated::*;\n".to_string())
}

/// Get a list of all available component templates
pub fn get_all_component_templates() -> Vec<&'static str> {
    let mut templates = supported_components()
        .into_iter()
        .filter(|&c| c != "all")
        .collect::<Vec<&'static str>>();
    templates.sort();
    templates
}
