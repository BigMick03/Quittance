//! Failure and error case tests for `payout-memo-binding`.
//!
//! These tests cover all the ways memo binding can fail, ensuring each
//! error variant is triggered correctly and the detection order is respected.

use payout_memo_binding::{matches_required_memo, memo_binding_satisfied, MemoBindingError};
use soroban_sdk::{Env, Symbol, symbol_short};

#[test]
fn memo_required_but_none_provided() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    assert_eq!(
        matches_required_memo(&env, Some(expected.clone()), None),
        Err(MemoBindingError::MissingRequiredMemo)
    );
    assert!(!memo_binding_satisfied(&env, Some(expected), None));
}

#[test]
fn memo_required_wrong_memo_provided() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    let wrong = symbol_short!("WRONG");
    assert_eq!(
        matches_required_memo(&env, Some(expected.clone()), Some(wrong.clone())),
        Err(MemoBindingError::MemoMismatch)
    );
    assert!(!memo_binding_satisfied(&env, Some(expected), Some(wrong)));
}

#[test]
fn missing_required_memo_precedes_mismatch_in_order() {
    // When expected is Some and provided is None, MissingRequiredMemo fires
    // before we even check for mismatch
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    assert_eq!(
        matches_required_memo(&env, Some(expected), None),
        Err(MemoBindingError::MissingRequiredMemo)
    );
}

#[test]
fn mismatch_only_when_both_present_and_different() {
    // MemoMismatch only when both expected and provided are Some but differ
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    let wrong = symbol_short!("WRONG");
    assert_eq!(
        matches_required_memo(&env, Some(expected), Some(wrong)),
        Err(MemoBindingError::MemoMismatch)
    );
}

#[test]
fn no_error_when_expected_none_and_provided_none() {
    let env = Env::default();
    assert_eq!(matches_required_memo(&env, None, None), Ok(()));
}

#[test]
fn no_error_when_expected_none_and_provided_some() {
    let env = Env::default();
    let extra = symbol_short!("EXTRA");
    assert_eq!(matches_required_memo(&env, None, Some(extra)), Ok(()));
}

#[test]
fn all_error_variants_have_non_empty_messages() {
    for err in [
        MemoBindingError::MissingRequiredMemo,
        MemoBindingError::MemoMismatch,
        MemoBindingError::InternalError,
    ] {
        assert!(!err.message().is_empty(), "Error {:?} message is empty", err);
    }
}

#[test]
fn error_discriminants_are_unique() {
    let codes = [
        MemoBindingError::MissingRequiredMemo as u32,
        MemoBindingError::MemoMismatch as u32,
        MemoBindingError::InternalError as u32,
    ];
    let unique: std::collections::BTreeSet<u32> = codes.iter().copied().collect();
    assert_eq!(unique.len(), codes.len(), "discriminants must not collide");
}

#[test]
fn error_discriminants_match_expected_values() {
    assert_eq!(MemoBindingError::MissingRequiredMemo as u32, 1);
    assert_eq!(MemoBindingError::MemoMismatch as u32, 2);
    assert_eq!(MemoBindingError::InternalError as u32, 3);
}

#[test]
fn boolean_wrapper_false_for_all_error_cases() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    let wrong = symbol_short!("WRONG");

    // Missing required memo
    assert!(!memo_binding_satisfied(&env, Some(expected.clone()), None));
    // Memo mismatch
    assert!(!memo_binding_satisfied(&env, Some(expected), Some(wrong)));
}

#[test]
fn detection_order_documented_and_stable() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    let wrong = symbol_short!("WRONG");

    // 1. MissingRequiredMemo takes precedence (when expected=Some, provided=None)
    assert_eq!(
        matches_required_memo(&env, Some(expected.clone()), None),
        Err(MemoBindingError::MissingRequiredMemo)
    );

    // 2. MemoMismatch when both present but different
    assert_eq!(
        matches_required_memo(&env, Some(expected.clone()), Some(wrong.clone())),
        Err(MemoBindingError::MemoMismatch)
    );

    // 3. Ok(()) for all other cases
    assert_eq!(matches_required_memo(&env, None, None), Ok(()));
    assert_eq!(matches_required_memo(&env, None, Some(wrong.clone())), Ok(()));
    assert_eq!(matches_required_memo(&env, Some(expected.clone()), Some(expected)), Ok(()));
}

#[test]
fn memo_binding_error_trait_implementations() {
    let err = MemoBindingError::MemoMismatch;
    let _ = format!("{:?}", err); // Debug
    let _ = err.message(); // message()
    let _ = err as u32; // Discriminant access

    // Clone and Copy
    let err2 = err;
    let err3 = err;
    assert_eq!(err2, err3);

    // PartialEq
    assert_eq!(MemoBindingError::MemoMismatch, MemoBindingError::MemoMismatch);
    assert_ne!(MemoBindingError::MemoMismatch, MemoBindingError::MissingRequiredMemo);
}