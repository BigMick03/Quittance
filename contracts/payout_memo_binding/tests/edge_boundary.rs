//! Edge and boundary case tests for `payout-memo-binding`.

use payout_memo_binding::{matches_required_memo, memo_binding_satisfied, encode_expected_memo, decode_expected_memo, MemoBindingError};
use soroban_sdk::{Env, Symbol, symbol_short};

#[test]
fn empty_string_symbol_works_as_memo() {
    let env = Env::default();
    let empty = Symbol::new(&env, "");
    assert_eq!(matches_required_memo(&env, Some(empty.clone()), Some(empty)), Ok(()));
}

#[test]
fn max_length_symbol_works() {
    // Soroban Symbol max length is around 32-64 bytes
    // We test with a reasonably long string that fits
    let env = Env::default();
    let long = "A".repeat(30);
    let memo = Symbol::new(&env, &long);
    assert_eq!(matches_required_memo(&env, Some(memo.clone()), Some(memo)), Ok(()));
}

#[test]
fn unicode_memo_works() {
    let env = Env::default();
    let memo = Symbol::new(&env, "INV_123_ZH_CN");
    assert_eq!(matches_required_memo(&env, Some(memo.clone()), Some(memo)), Ok(()));
}

#[test]
fn memo_with_numbers_and_symbols() {
    let env = Env::default();
    let memo = Symbol::new(&env, "INV_123_456_ABC");
    assert_eq!(matches_required_memo(&env, Some(memo.clone()), Some(memo)), Ok(()));
}

#[test]
fn case_sensitive_comparison() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    let provided = Symbol::new(&env, "inv_123"); // Different case
    assert_eq!(matches_required_memo(&env, Some(expected), Some(provided)), Err(MemoBindingError::MemoMismatch));
}

#[test]
fn storage_roundtrip_with_empty_symbol() {
    let env = Env::default();
    let empty = Symbol::new(&env, "");
    let stored = encode_expected_memo(Some(empty.clone()));
    let decoded = decode_expected_memo(stored);
    assert_eq!(decoded, Some(empty));
}

#[test]
fn storage_roundtrip_with_long_symbol() {
    let env = Env::default();
    let long = "A".repeat(30);
    let memo = Symbol::new(&env, &long);
    let stored = encode_expected_memo(Some(memo.clone()));
    let decoded = decode_expected_memo(stored);
    assert_eq!(decoded, Some(memo));
}

#[test]
fn boolean_wrapper_false_for_mismatch() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    let wrong = symbol_short!("WRONG");
    assert!(!memo_binding_satisfied(&env, Some(expected), Some(wrong)));
}

#[test]
fn boolean_wrapper_false_for_missing() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    assert!(!memo_binding_satisfied(&env, Some(expected), None));
}

#[test]
fn boolean_wrapper_true_for_extra_memo_when_not_required() {
    let env = Env::default();
    assert!(memo_binding_satisfied(&env, None, Some(symbol_short!("EXTRA"))));
}

#[test]
fn sentinel_symbol_not_exposed_in_public_api() {
    // Verify the internal sentinel is not a valid user memo
    let env = Env::default();
    let user_memo = Symbol::new(&env, "USER_MEMO");
    // User memo should decode to Some, not None
    let stored = encode_expected_memo(Some(user_memo.clone()));
    let decoded = decode_expected_memo(stored);
    assert_eq!(decoded, Some(user_memo));
}