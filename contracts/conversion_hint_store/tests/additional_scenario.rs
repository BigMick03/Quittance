//! Additional scenario tests for `conversion-hint-store`.
//!
//! These tests cover specific scenarios relevant to conversion hint validation
//! on Stellar, such as multiple currency pairs, rate updates, and integration patterns.

use conversion_hint_store::{validate_currency, validate_hint_entry, hint_key, disclaimer_key, is_non_authoritative};

#[test]
fn multiple_currency_pairs_independent() {
    // Each currency pair validates independently
    assert!(validate_hint_entry("USD", "XLM", 105000000).is_ok());
    assert!(validate_hint_entry("EUR", "XLM", 115000000).is_ok());
    assert!(validate_hint_entry("GBP", "XLM", 130000000).is_ok());
    assert!(validate_hint_entry("JPY", "XLM", 700000).is_ok());
}

#[test]
fn reverse_pair_is_independent() {
    // USD -> XLM and XLM -> USD are independent
    assert!(validate_hint_entry("USD", "XLM", 105000000).is_ok());
    assert!(validate_hint_entry("XLM", "USD", 9523810).is_ok()); // ~1/10.5
}

#[test]
fn rate_update_scenario() {
    // Simulate rate updates throughout the day
    let rates = [105000000, 106000000, 104500000, 107000000];
    for rate in rates {
        assert!(validate_hint_entry("USD", "XLM", rate).is_ok());
    }
}

#[test]
fn standard_iso_currency_codes() {
    // Test standard ISO 4217 codes
    let codes = ["USD", "EUR", "GBP", "JPY", "CHF", "CAD", "AUD", "CNY"];
    for code in codes {
        assert!(validate_currency(code).is_ok());
        assert!(validate_hint_entry(code, "XLM", 100000000).is_ok());
    }
}

#[test]
fn custom_token_symbols() {
    // Custom token symbols (up to 12 chars)
    assert!(validate_currency("TOKEN").is_ok());
    assert!(validate_currency("MYTOKEN").is_ok());
    assert!(validate_currency("TOKEN123").is_ok());
    assert!(validate_hint_entry("MYTOKEN", "XLM", 50000000).is_ok());
}

#[test]
fn stablecoin_pairs() {
    // Common stablecoin pairs
    assert!(validate_hint_entry("USDC", "XLM", 100000000).is_ok()); // 1 USDC ≈ 1 USD
    assert!(validate_hint_entry("USDT", "XLM", 100000000).is_ok());
    assert!(validate_hint_entry("DAI", "XLM", 100000000).is_ok());
}

#[test]
fn cross_environment_key_consistency() {
    // Storage key generation is consistent
    assert_eq!(hint_key("USD", "XLM"), "HINT_USD_XLM");
    assert_eq!(hint_key("usd", "xlm"), "HINT_USD_XLM");
    assert_eq!(disclaimer_key("USD", "XLM"), "HINT_USD_XLM_DISCLAIMER");
}

#[test]
fn non_authoritative_flag_always_true() {
    // The non-authoritative flag is always true by design
    assert!(is_non_authoritative());
    // This is a constant, so it's always true
    assert!(is_non_authoritative());
}