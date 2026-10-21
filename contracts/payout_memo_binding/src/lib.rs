//! `payout-memo-binding` — Payout memo binding check.
//!
//! Validates that a payout destination's required memo (if any) matches the
//! expected memo before release.
//!
//! This crate is a pure Soroban helper. It is intentionally minimal and
//! focuses on a single primitive: comparing an optional expected memo against
//! an optional provided memo. It can be used both on-chain (inside a Soroban
//! contract) and off-chain (in the Express backend, CLI tools, or tests).
//!
//! This crate is **not** wired into the Next.js or Express MVP demos in this
//! PR. It is purely the validation primitive that future payout work will call
//! into.
//!
//! # What it does
//!
//! Given an optional expected memo (what the destination requires) and an
//! optional provided memo (what the payment includes), it returns `Ok(())` if
//! the binding is satisfied, or a [`MemoBindingError`] indicating the
//! violation.
//!
//! # Rules
//!
//! 1. **No memo required** (`expected = None`): Any provided memo is accepted
//!    (including `None`). The destination doesn't care about memos.
//! 2. **Memo required** (`expected = Some(memo)`): The provided memo must be
//!    `Some(expected_memo)` — exact match required.
//!
//! # Detection order
//!
//! `matches_required_memo` checks conditions in a fixed order:
//!
//! 1. **Missing required memo** — expected is `Some` but provided is `None`.
//! 2. **Memo mismatch** — expected is `Some` and provided is `Some` but they
//!    differ.
//! 3. **Satisfied** — all other cases (including `expected = None`).
//!
//! The order is part of the public contract so consumers can rely on
//! deterministic error precedence in diagnostics and tests.
//!
//! # Public API
//!
//! | Item | Type | Meaning |
//! |------|------|---------|
//! | `matches_required_memo(env, expected, provided)` | `fn(&Env, Option<Symbol>, Option<Symbol>) -> MemoBindingResult` | Primary validation entry point. |
//! | `memo_binding_satisfied(env, expected, provided)` | `fn(&Env, Option<Symbol>, Option<Symbol>) -> bool` | Boolean convenience wrapper. |
//! | `encode_expected_memo(expected)` | `fn(Option<Symbol>) -> Symbol` | Encode for contract storage. |
//! | `decode_expected_memo(stored)` | `fn(Symbol) -> Option<Symbol>` | Decode from contract storage. |
//! | `MemoBindingError` | `enum` | Stable error codes with `#[repr(u32)]` discriminants. |
//!
//! # Usage
//!
//! ```rust
//! use payout_memo_binding::{matches_required_memo, memo_binding_satisfied, MemoBindingError, encode_expected_memo, decode_expected_memo};
//! use soroban_sdk::{Env, Symbol, symbol_short};
//!
//! let env = Env::default();
//! let expected_memo = symbol_short!("INV_123");
//!
//! // No memo required, no memo provided -> OK
//! assert!(matches_required_memo(&env, None, None).is_ok());
//!
//! // No memo required, memo provided -> OK (extra memo allowed)
//! assert!(matches_required_memo(&env, None, Some(symbol_short!("extra"))).is_ok());
//!
//! // Memo required, correct memo provided -> OK
//! assert!(matches_required_memo(&env, Some(expected_memo.clone()), Some(expected_memo.clone())).is_ok());
//!
//! // Memo required, no memo provided -> Error
//! assert_eq!(
//!     matches_required_memo(&env, Some(expected_memo.clone()), None),
//!     Err(MemoBindingError::MissingRequiredMemo)
//! );
//!
//! // Memo required, wrong memo provided -> Error
//! assert_eq!(
//!     matches_required_memo(&env, Some(expected_memo.clone()), Some(symbol_short!("WRONG"))),
//!     Err(MemoBindingError::MemoMismatch)
//! );
//!
//! // Boolean convenience wrapper
//! assert!(memo_binding_satisfied(&env, None, None));
//! assert!(!memo_binding_satisfied(&env, Some(expected_memo.clone()), None));
//!
//! // Storage encoding/decoding
//! let stored = encode_expected_memo(Some(expected_memo.clone()));
//! assert_eq!(decode_expected_memo(stored), Some(expected_memo));
//! let stored_none = encode_expected_memo(None);
//! assert_eq!(decode_expected_memo(stored_none), None);
//! ```
//!
//! # Running the tests
//!
//! This crate requires the Soroban SDK. Install Rust from <https://rustup.rs>
//! if missing, then:
//!
//! ```bash
//! cd contracts/payout_memo_binding
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

pub use crate::types::{MemoBindingError, MemoBindingResult};
pub use crate::validate::{decode_expected_memo, encode_expected_memo, matches_required_memo, memo_binding_satisfied};