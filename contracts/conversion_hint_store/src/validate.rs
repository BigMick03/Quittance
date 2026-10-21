//! Core validation logic for the conversion hint store.

use crate::types::{ConversionHintError, ConversionHintResult, MAX_CURRENCY_LEN, NON_AUTHORITATIVE};

/// Validate a currency symbol.
/// - Must not be empty
/// - Must not exceed MAX_CURRENCY_LEN
pub fn validate_currency(currency: &str) -> ConversionHintResult {
    let len = currency.len() as u32;
    if len == 0 {
        return Err(ConversionHintError::InvalidCurrency);
    }
    if len > MAX_CURRENCY_LEN {
        return Err(ConversionHintError::InvalidCurrency);
    }
    Ok(())
}

/// Validate a conversion rate.
/// Must be positive (greater than zero) when expressed in e7 (7 decimal places).
pub fn validate_rate(rate_e7: i128) -> ConversionHintResult {
    if rate_e7 <= 0 {
        return Err(ConversionHintError::InvalidRate);
    }
    Ok(())
}

/// Check if a hint is non-authoritative.
/// This always returns `true` since all hints in this store are explicitly
/// non-authoritative by design.
pub fn is_non_authoritative() -> bool {
    NON_AUTHORITATIVE
}

/// Generate a storage key for a currency pair.
/// Format: "HINT_{from}_{to}" (e.g., "HINT_USD_XLM")
pub fn hint_key(from: &str, to: &str) -> String {
    format!("HINT_{}_{}", from.to_uppercase(), to.to_uppercase())
}

/// Generate a disclaimer storage key for a currency pair.
/// Format: "HINT_{from}_{to}_DISCLAIMER"
pub fn disclaimer_key(from: &str, to: &str) -> String {
    format!("{}_DISCLAIMER", hint_key(from, to))
}

/// Validate a complete hint entry (currency pair + rate).
/// This is the main validation function that consuming contracts should call
/// before storing a hint.
pub fn validate_hint_entry(from: &str, to: &str, rate_e7: i128) -> ConversionHintResult {
    validate_currency(from)?;
    validate_currency(to)?;
    validate_rate(rate_e7)?;
    Ok(())
}