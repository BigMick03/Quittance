//! Failure and error case tests for `conversion-hint-store`.

use conversion_hint_store::{validate_currency, validate_rate, validate_hint_entry, ConversionHintError};

#[test]
fn validate_currency_rejects_empty() {
    assert_eq!(validate_currency(""), Err(ConversionHintError::InvalidCurrency));
}

#[test]
fn validate_currency_rejects_too_long() {
    let too_long = "A".repeat(13);
    assert_eq!(validate_currency(&too_long), Err(ConversionHintError::InvalidCurrency));
}

#[test]
fn validate_rate_rejects_zero() {
    assert_eq!(validate_rate(0), Err(ConversionHintError::InvalidRate));
}

#[test]
fn validate_rate_rejects_negative() {
    assert_eq!(validate_rate(-1), Err(ConversionHintError::InvalidRate));
    assert_eq!(validate_rate(-10000000), Err(ConversionHintError::InvalidRate));
}

#[test]
fn validate_hint_entry_rejects_invalid_from() {
    assert_eq!(validate_hint_entry("", "XLM", 105000000), Err(ConversionHintError::InvalidCurrency));
}

#[test]
fn validate_hint_entry_rejects_invalid_to() {
    assert_eq!(validate_hint_entry("USD", "", 105000000), Err(ConversionHintError::InvalidCurrency));
}

#[test]
fn validate_hint_entry_rejects_zero_rate() {
    assert_eq!(validate_hint_entry("USD", "XLM", 0), Err(ConversionHintError::InvalidRate));
}

#[test]
fn validate_hint_entry_rejects_negative_rate() {
    assert_eq!(validate_hint_entry("USD", "XLM", -100), Err(ConversionHintError::InvalidRate));
}

#[test]
fn all_error_variants_have_non_empty_messages() {
    for err in [
        ConversionHintError::InvalidRate,
        ConversionHintError::InvalidCurrency,
    ] {
        assert!(!err.message().is_empty(), "Error {:?} message is empty", err);
    }
}

#[test]
fn error_discriminants_are_unique() {
    let codes = [
        ConversionHintError::InvalidRate as u32,
        ConversionHintError::InvalidCurrency as u32,
    ];
    let unique: std::collections::BTreeSet<u32> = codes.iter().copied().collect();
    assert_eq!(unique.len(), codes.len());
}

#[test]
fn error_discriminants_match_expected_values() {
    assert_eq!(ConversionHintError::InvalidRate as u32, 1);
    assert_eq!(ConversionHintError::InvalidCurrency as u32, 2);
}

#[test]
fn error_trait_implementations() {
    let err = ConversionHintError::InvalidRate;
    let _ = format!("{:?}", err);
    let _ = err.message();
    let _ = err as u32;
    let err2 = err;
    let err3 = err;
    assert_eq!(err2, err3);
    assert_eq!(ConversionHintError::InvalidRate, ConversionHintError::InvalidRate);
    assert_ne!(ConversionHintError::InvalidRate, ConversionHintError::InvalidCurrency);
}