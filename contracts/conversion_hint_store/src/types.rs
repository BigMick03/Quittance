//! Type definitions for the conversion-hint-store crate.

/// Error codes returned by the conversion hint validation.
///
/// Each variant carries a stable `#[repr(u32)]` discriminant so off-chain
/// tooling can match on the numeric code without depending on the Rust
/// identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ConversionHintError {
    /// The provided rate is invalid (e.g., zero or negative).
    InvalidRate = 1,
    /// The currency symbol is invalid (empty or too long).
    InvalidCurrency = 2,
}

impl ConversionHintError {
    /// Return a stable English message describing the error.
    pub fn message(&self) -> &'static str {
        match self {
            ConversionHintError::InvalidRate => "Conversion rate must be positive (greater than zero).",
            ConversionHintError::InvalidCurrency => "Currency symbol is invalid (empty or too long).",
        }
    }
}

/// Result type for conversion hint validation.
pub type ConversionHintResult = Result<(), ConversionHintError>;

/// Maximum allowed length for a currency symbol.
/// ISO 4217 codes are 3 characters, but we allow some flexibility for custom tokens.
pub const MAX_CURRENCY_LEN: u32 = 12;

/// Non-authoritative disclaimer flag.
/// All hints in this store are explicitly non-authoritative by design.
pub const NON_AUTHORITATIVE: bool = true;