//! Edge and boundary case tests for `conversion-hint-store`.

use conversion_hint_store::{validate_currency, validate_rate, validate_hint_entry};

#[test]
fn validate_currency_accepts_max_length() {
    let max_len = "A".repeat(12);
    assert!(validate_currency(&max_len).is_ok());
}

#[test]
fn validate_currency_rejects_empty() {
    assert_eq!(validate_currency(""), Err(conversion_hint_store::ConversionHintError::InvalidCurrency));
}

#[test]
fn validate_currency_rejects_too_long() {
    let too_long = "A".repeat(13);
    assert_eq!(validate_currency(&too_long), Err(conversion_hint_store::ConversionHintError::InvalidCurrency));
}

#[test]
fn validate_currency_case_sensitive() {
    // "usd" and "USD" are both valid but different
    assert!(validate_currency("usd").is_ok());
    assert!(validate_currency("USD").is_ok());
}

#[test]
fn validate_rate_accepts_minimum_positive() {
    assert!(validate_rate(1).is_ok()); // Minimum positive in e7
}

#[test]
fn validate_rate_accepts_large_values() {
    let large_rate = 1_000_000 * 10_000_000; // 10 trillion in e7
    assert!(validate_rate(large_rate).is_ok());
    assert!(validate_rate(i128::MAX).is_ok());
}

#[test]
fn validate_hint_entry_rejects_invalid_from() {
    assert_eq!(validate_hint_entry("", "XLM", 105000000), Err(conversion_hint_store::ConversionHintError::InvalidCurrency));
}

#[test]
fn validate_hint_entry_rejects_invalid_to() {
    assert_eq!(validate_hint_entry("USD", "", 105000000), Err(conversion_hint_store::ConversionHintError::InvalidCurrency));
}

#[test]
fn validate_hint_entry_rejects_invalid_rate() {
    assert_eq!(validate_hint_entry("USD", "XLM", 0), Err(conversion_hint_store::ConversionHintError::InvalidRate));
    assert_eq!(validate_hint_entry("USD", "XLM", -100), Err(conversion_hint_store::ConversionHintError::InvalidRate));
}