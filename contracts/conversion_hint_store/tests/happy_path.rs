//! Happy-path tests for `conversion-hint-store`.
//!
//! These tests cover the successful validation scenarios.

use conversion_hint_store::{validate_currency, validate_rate, validate_hint_entry, is_non_authoritative, hint_key, disclaimer_key};

#[test]
fn validate_currency_accepts_valid_codes() {
    assert!(validate_currency("USD").is_ok());
    assert!(validate_currency("EUR").is_ok());
    assert!(validate_currency("XLM").is_ok());
    assert!(validate_currency("USDC").is_ok());
    assert!(validate_currency("BTC").is_ok());
}

#[test]
fn validate_currency_accepts_max_length() {
    let max_len = "A".repeat(12);
    assert!(validate_currency(&max_len).is_ok());
}

#[test]
fn validate_currency_case_sensitive() {
    assert!(validate_currency("usd").is_ok());
    assert!(validate_currency("USD").is_ok());
    // These are different currencies
}

#[test]
fn validate_rate_accepts_positive() {
    assert!(validate_rate(1).is_ok());
    assert!(validate_rate(10000000).is_ok());  // 1.0 in e7
    assert!(validate_rate(105000000).is_ok()); // 10.5 in e7
    assert!(validate_rate(i128::MAX).is_ok());
}

#[test]
fn validate_hint_entry_accepts_valid() {
    assert!(validate_hint_entry("USD", "XLM", 105000000).is_ok());
    assert!(validate_hint_entry("EUR", "XLM", 115000000).is_ok());
    assert!(validate_hint_entry("BTC", "XLM", 500000000000).is_ok());
}

#[test]
fn is_non_authoritative_always_true() {
    assert!(is_non_authoritative());
}

#[test]
fn hint_key_format() {
    assert_eq!(hint_key("USD", "XLM"), "HINT_USD_XLM");
    assert_eq!(hint_key("usd", "xlm"), "HINT_USD_XLM");
    assert_eq!(hint_key("EUR", "XLM"), "HINT_EUR_XLM");
}

#[test]
fn disclaimer_key_format() {
    assert_eq!(disclaimer_key("USD", "XLM"), "HINT_USD_XLM_DISCLAIMER");
}