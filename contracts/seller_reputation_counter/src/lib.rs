//! `seller-reputation-counter` — Seller reputation counter.
//!
//! Validation logic for a view-only counter tracking successful vs. disputed
//! invoice counts per seller.
//!
//! This crate is a **pure Rust library** (no external dependencies) for
//! computing seller reputation scores. It is intentionally dependency-free
//! so it can be used both on-chain (inside a Soroban contract via `no_std` +
//! `alloc`) and off-chain (in the Express backend, CLI tools, or tests)
//! without pulling in the Soroban SDK or any cryptographic primitives.
//!
//! This crate is **not** wired into the Next.js or Express MVP demos in this
//! PR. It is purely the reputation primitive that future marketplace work
//! will call into.
//!
//! # What it does
//!
//! - Tracks **successful invoices** and **disputed invoices** per seller.
//! - Provides derived metrics: total invoices, success rate percentage.
//! - Overflow-safe with saturating arithmetic.
//!
//! # Public API
//!
//! | Item | Type | Meaning |
//! |------|------|---------|
//! | `record_success(current)` | `fn(ReputationScore) -> ReputationResult` | Increment success count. |
//! | `record_dispute(current)` | `fn(ReputationScore) -> ReputationResult` | Increment dispute count. |
//! | `total_invoices(score)` | `fn(ReputationScore) -> u32` | Get total invoice count. |
//! | `success_rate(score)` | `fn(ReputationScore) -> Option<u32>` | Get success rate (0-100). |
//! | `reputation_key(seller)` | `fn(&str) -> String` | Generate storage key for seller. |
//! | `validate_seller(seller)` | `fn(&str) -> ReputationResult` | Validate seller address. |
//! | `ReputationError` | `enum` | Stable error codes with `#[repr(u32)]` discriminants. |
//! | `ReputationScore` | `type` | Tuple of `(success_count, dispute_count)`. |
//!
//! # Usage
//!
//! ```rust
//! use seller_reputation_counter::{record_success, record_dispute, total_invoices, success_rate, ReputationScore, ReputationError};
//!
//! // Start with no reputation
//! let mut score: ReputationScore = (0, 0);
//!
//! // Record some outcomes
//! score = record_success(score).unwrap();
//! score = record_success(score).unwrap();
//! score = record_dispute(score).unwrap();
//!
//! // Query reputation
//! assert_eq!(score, (2, 1));
//! assert_eq!(total_invoices(score), 3);
//! assert_eq!(success_rate(score), Some(66)); // 2/3 = 66%
//! ```
//!
//! # Running the tests
//!
//! This crate has no external dependencies, so it only needs a standard Rust
//! toolchain (1.74+ recommended). Install from <https://rustup.rs> if missing:
//!
//! ```bash
//! cd contracts/seller_reputation_counter
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

pub use crate::types::{ReputationError, ReputationResult, ReputationScore, MAX_COUNT};
pub use crate::validate::{record_dispute, record_success, reputation_key, success_rate, total_invoices, validate_seller};