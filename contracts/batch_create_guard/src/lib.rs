//! `batch-create-guard` — Batch invoice creation guard.
//!
//! Validates a batch of invoice-creation requests against a maximum batch
//! size and a maximum total amount.
//!
//! This crate is a pure Rust helper with no external dependencies. It is
//! intentionally dependency-free so it can be used both on-chain (inside a
//! Soroban contract via `no_std` + `alloc`) and off-chain (in the Express
//! backend, CLI tools, or tests) without pulling in the Soroban SDK or any
//! cryptographic primitives.
//!
//! # What it does
//!
//! Given a list of invoice amounts (in stroops, Stellar's smallest on-chain
//! unit) and two policy parameters — `max_count` (maximum invoices per batch)
//! and `max_total` (maximum sum of all amounts in the batch) — it returns
//! `Ok(())` if the batch passes all checks, or a [`BatchError`] indicating
//! the first violation detected.
//!
//! # Detection order
//!
//! `validate_batch` checks conditions in a fixed order. The first failure is
//! returned:
//!
//! 1. **Invalid max_count** — `max_count` is zero.
//! 2. **Invalid max_total** — `max_total` is negative.
//! 3. **Empty batch** — the amounts slice is empty.
//! 4. **Count exceeded** — batch length exceeds `max_count`.
//! 5. **Negative amount** — any individual amount is negative.
//! 6. **Zero amount** — any individual amount is zero.
//! 7. **Total exceeded** — sum of all amounts exceeds `max_total`.
//!
//! The order is part of the public contract so consumers can rely on
//! deterministic error precedence in diagnostics and tests.
//!
//! # Public API
//!
//! | Item | Type | Meaning |
//! |------|------|---------|
//! | `validate_batch(amounts, max_count, max_total)` | `fn(&[i128], u32, i128) -> BatchResult` | Primary validation entry point. |
//! | `validate_batch_with_config(amounts, config)` | `fn(&[i128], &BatchConfig) -> BatchResult` | Convenience wrapper using a pre-validated config. |
//! | `is_valid_batch(amounts, max_count, max_total)` | `fn(&[i128], u32, i128) -> bool` | Boolean convenience wrapper. |
//! | `BatchConfig::new(max_count, max_total)` | `fn(u32, i128) -> Result<BatchConfig, BatchError>` | Construct a validated config object. |
//! | `BatchError` | `enum` | Stable error codes with `#[repr(u32)]` discriminants. |
//!
//! # Usage
//!
//! ```rust
//! use batch_create_guard::{validate_batch, BatchError, is_valid_batch, BatchConfig};
//!
//! // 3 invoices at 10 XLM each (stroops), max 5 invoices, max 100 XLM total
//! let amounts = [10_000_000_000i128; 3];
//!
//! // Primary API — returns detailed error on failure
//! let result = validate_batch(&amounts, 5, 100_000_000_000);
//! assert!(result.is_ok());
//!
//! // Count exceeded
//! let too_many = [1_000_000_000i128; 6];
//! assert_eq!(
//!     validate_batch(&too_many, 5, 100_000_000_000),
//!     Err(BatchError::CountExceeded)
//! );
//!
//! // Boolean convenience wrapper
//! assert!(is_valid_batch(&amounts, 5, 100_000_000_000));
//! assert!(!is_valid_batch(&too_many, 5, 100_000_000_000));
//!
//! // Config object for repeated validation with same policy
//! let config = BatchConfig::new(5, 100_000_000_000).unwrap();
//! assert!(batch_create_guard::validate_batch_with_config(&amounts, &config).is_ok());
//! ```
//!
//! # Running the tests
//!
//! This crate has no external dependencies, so it only needs a standard Rust
//! toolchain (1.74+ recommended). Install from <https://rustup.rs> if missing:
//!
//! ```bash
//! cd contracts/batch_create_guard
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

pub use crate::types::{BatchConfig, BatchError, BatchResult};
pub use crate::validate::{is_valid_batch, validate_batch, validate_batch_with_config};