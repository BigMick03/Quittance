//! Type definitions for the payout-memo-binding crate.

use soroban_sdk::{Symbol, symbol_short};

/// Error codes returned by [`matches_required_memo`].
///
/// Each variant carries a stable `#[repr(u32)]` discriminant so off-chain
/// tooling can match on the numeric code without depending on the Rust
/// identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum MemoBindingError {
    /// The payout destination requires a memo but none was provided.
    MissingRequiredMemo = 1,
    /// The provided memo does not match the expected memo.
    MemoMismatch = 2,
    /// Internal error: unexpected state (should not occur with valid inputs).
    InternalError = 3,
}

impl MemoBindingError {
    /// Return a stable English message describing the error.
    pub fn message(&self) -> &'static str {
        match self {
            MemoBindingError::MissingRequiredMemo => "Payout destination requires a memo but none was provided.",
            MemoBindingError::MemoMismatch => "Provided memo does not match the expected memo.",
            MemoBindingError::InternalError => "Internal error during memo binding check.",
        }
    }
}

/// Result type for memo binding validation.
pub type MemoBindingResult = Result<(), MemoBindingError>;

/// Internal sentinel for "no memo expected" — using an impossible symbol value.
/// We use a private constant rather than `None` in the public API to make
/// the contract storage representation explicit.
pub(crate) const NO_MEMO_EXPECTED: Symbol = symbol_short!("NO_MEMO");