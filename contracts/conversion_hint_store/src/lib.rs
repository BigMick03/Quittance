//! `conversion-hint-store` — Non-authoritative conversion hint store.
//!
//! Validation logic for display-only currency conversion hint rates
//! with an explicit 'not authoritative' disclaimer flag.
//!
//! This crate is a **pure Rust library** (no external dependencies) for
//! validating currency conversion hint rates. It is intentionally
//! dependency-free so it can be used both on-chain (inside a Soroban
//! contract via `no_std` + `alloc`) and off-chain (in the Express
//! backend, CLI tools, or tests) without pulling in the Soroban SDK
//! or any cryptographic primitives.
//!
//! This crate is **not** wired into the Next.js or Express MVP demos in
//! this PR. It is purely the validation primitive that future conversion
//! display work will call into.
//!
//! # What it does
//!
//! Provides validation functions for currency conversion hints that are
//! explicitly marked as **non-authoritative** — they are intended for
//! display purposes only (e.g., showing approximate fiat values in a UI)
//! and must not be used for financial settlement, accounting, or any
//! authoritative purpose.
//!
//! # Non-Authoritative Disclaimer
//!
//! **All rates validated by this library are non-authoritative.** They are:
//! - **Display hints only** — for UI estimation, not settlement.
//! - **Not guaranteed accurate** — may be stale, approximate, or wrong.
//! - **Not for financial use** — never use for invoicing, payment calculation, or accounting.
//!
//! The `is_non_authoritative()` function always returns `true`, reinforcing
//! this design intent.
//!
//! # Public API
//!
//! | Item | Type | Meaning |
//! |------|------|---------|
//! | `validate_currency(currency)` | `fn(&str) -> ConversionHintResult` | Validates a currency symbol. |
//! | `validate_rate(rate_e7)` | `fn(i128) -> ConversionHintResult` | Validates a rate in e7 format. |
//! | `validate_hint_entry(from, to, rate_e7)` | `fn(&str, &str, i128) -> ConversionHintResult` | Validates a complete hint entry. |
//! | `is_non_authoritative()` | `fn() -> bool` | Always returns `true` (explicit disclaimer). |
//! | `hint_key(from, to)` | `fn(&str, &str) -> String` | Generates storage key for a pair. |
//! | `disclaimer_key(from, to)` | `fn(&str, &str) -> String` | Generates disclaimer key for a pair. |
//! | `ConversionHintError` | `enum` | Stable error codes with `#[repr(u32)]` discriminants. |
//!
//! # Usage
//!
//! ```rust
//! use conversion_hint_store::{validate_hint_entry, validate_currency, validate_rate, is_non_authoritative, hint_key, ConversionHintError};
//!
//! // Validate a complete hint entry
//! assert!(validate_hint_entry("USD", "XLM", 105000000).is_ok());
//! assert_eq!(validate_hint_entry("", "XLM", 105000000), Err(ConversionHintError::InvalidCurrency));
//! assert_eq!(validate_hint_entry("USD", "XLM", 0), Err(ConversionHintError::InvalidRate));
//!
//! // Individual validators
//! assert!(validate_currency("USD").is_ok());
//! assert!(validate_currency("").is_err());
//! assert!(validate_rate(105000000).is_ok());
//! assert!(validate_rate(0).is_err());
//!
//! // Non-authoritative disclaimer
//! assert!(is_non_authoritative());
//!
//! // Storage key helpers
//! assert_eq!(hint_key("USD", "XLM"), "HINT_USD_XLM");
//! ```
//!
//! # Running the tests
//!
//! This crate has no external dependencies, so it only needs a standard Rust
//! toolchain (1.74+ recommended). Install from <https://rustup.rs> if missing:
//!
//! ```bash
//! cd contracts/conversion_hint_store
//! cargo test
//! cargo clippy --all-targets -- -D warnings
//! ```
//!
//! # License
//!
//! MIT — see `LICENSE` at the repository root.

#![deny(missing_docs)]
#![deny(unsafe_code)]
#![forbid(unused_must_use)]

mod types;
mod validate;

pub use crate::types::{ConversionHintError, ConversionHintResult, MAX_CURRENCY_LEN, NON_AUTHORITATIVE};
pub use crate::validate::{disclaimer_key, hint_key, is_non_authoritative, validate_currency, validate_hint_entry, validate_rate};