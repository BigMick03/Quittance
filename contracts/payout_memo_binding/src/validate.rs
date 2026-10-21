//! Core validation logic for payout memo binding.

use crate::types::{MemoBindingError, MemoBindingResult, NO_MEMO_EXPECTED};
use soroban_sdk::{Env, Symbol};

/// Check if the provided memo matches the required memo for a payout destination.
///
/// This function validates the memo binding rule:
/// - If the destination requires a memo (`expected` is `Some`), the caller
///   must provide exactly that memo (`provided == expected`).
/// - If the destination does not require a memo (`expected` is `None`),
///   any provided memo is accepted (including `None`).
///
/// # Arguments
///
/// * `env` — Soroban environment (for Symbol comparison).
/// * `expected` — The memo required by the destination, if any. `None` means
///   no memo is required.
/// * `provided` — The memo included in the payment transaction, if any.
///
/// # Examples
///
/// ```rust
/// use payout_memo_binding::matches_required_memo;
/// use soroban_sdk::{Env, Symbol, symbol_short};
///
/// let env = Env::default();
///
/// // No memo required, no memo provided -> OK
/// assert!(matches_required_memo(&env, None, None).is_ok());
///
/// // No memo required, memo provided -> OK (extra memo allowed)
/// assert!(matches_required_memo(&env, None, Some(symbol_short!("extra"))).is_ok());
///
/// // Memo required, correct memo provided -> OK
/// let expected = symbol_short!("INV_123");
/// assert!(matches_required_memo(&env, Some(expected.clone()), Some(expected.clone())).is_ok());
///
/// // Memo required, no memo provided -> Error
/// assert!(matches_required_memo(&env, Some(expected.clone()), None).is_err());
///
/// // Memo required, wrong memo provided -> Error
/// assert!(matches_required_memo(&env, Some(expected), Some(symbol_short!("WRONG"))).is_err());
/// ```
pub fn matches_required_memo(_env: &Env, expected: Option<Symbol>, provided: Option<Symbol>) -> MemoBindingResult {
    match expected {
        Some(expected_memo) => {
            // Destination requires a specific memo
            match provided {
                Some(provided_memo) => {
                    if provided_memo == expected_memo {
                        Ok(())
                    } else {
                        Err(MemoBindingError::MemoMismatch)
                    }
                }
                None => {
                    // Required memo but none provided
                    Err(MemoBindingError::MissingRequiredMemo)
                }
            }
        }
        None => {
            // No memo required — accept anything (including None)
            Ok(())
        }
    }
}

/// Convenience function returning `true` when [`matches_required_memo`] returns `Ok(())`.
///
/// Mirrors the `is_valid_destination` pattern in `destination_guard`: callers
/// that only need a yes/no answer can skip handling the [`MemoBindingError`] enum.
///
/// # Examples
///
/// ```rust
/// use payout_memo_binding::memo_binding_satisfied;
/// use soroban_sdk::{Env, Symbol, symbol_short};
///
/// let env = Env::default();
/// let expected = symbol_short!("INV_123");
///
/// assert!(memo_binding_satisfied(&env, None, None));
/// assert!(memo_binding_satisfied(&env, None, Some(symbol_short!("extra"))));
/// assert!(memo_binding_satisfied(&env, Some(expected.clone()), Some(expected.clone())));
/// assert!(!memo_binding_satisfied(&env, Some(expected.clone()), None));
/// assert!(!memo_binding_satisfied(&env, Some(expected), Some(symbol_short!("WRONG"))));
/// ```
#[must_use = "the validation result is ignored; consider using `matches_required_memo` if you need to distinguish the error reason"]
pub fn memo_binding_satisfied(env: &Env, expected: Option<Symbol>, provided: Option<Symbol>) -> bool {
    matches_required_memo(env, expected, provided).is_ok()
}

/// Encode the expected memo for storage.
///
/// Converts `Option<Symbol>` to a `Symbol` suitable for contract storage:
/// - `Some(memo)` -> `memo`
/// - `None` -> `NO_MEMO_EXPECTED` sentinel
///
/// This makes the "no memo required" state explicit in storage rather than
/// relying on optional presence.
pub fn encode_expected_memo(expected: Option<Symbol>) -> Symbol {
    expected.unwrap_or(NO_MEMO_EXPECTED)
}

/// Decode the expected memo from storage.
///
/// Converts a stored `Symbol` back to `Option<Symbol>`:
/// - `NO_MEMO_EXPECTED` -> `None`
/// - Any other symbol -> `Some(symbol)`
pub fn decode_expected_memo(stored: Symbol) -> Option<Symbol> {
    if stored == NO_MEMO_EXPECTED {
        None
    } else {
        Some(stored)
    }
}