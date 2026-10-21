//! Happy-path tests for `payout-memo-binding`.
//!
//! These tests cover the successful validation scenarios where memo
//! binding is satisfied.

use payout_memo_binding::{matches_required_memo, memo_binding_satisfied, encode_expected_memo, decode_expected_memo};
use soroban_sdk::{Env, Symbol, symbol_short};

#[test]
fn no_memo_required_no_memo_provided() {
    let env = Env::default();
    assert_eq!(matches_required_memo(&env, None, None), Ok(()));
    assert!(memo_binding_satisfied(&env, None, None));
}

#[test]
fn no_memo_required_memo_provided_allowed() {
    let env = Env::default();
    let extra = symbol_short!("EXTRA");
    assert_eq!(matches_required_memo(&env, None, Some(extra.clone())), Ok(()));
    assert!(memo_binding_satisfied(&env, None, Some(extra)));
}

#[test]
fn memo_required_correct_memo_provided() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    assert_eq!(matches_required_memo(&env, Some(expected.clone()), Some(expected.clone())), Ok(()));
    assert!(memo_binding_satisfied(&env, Some(expected.clone()), Some(expected)));
}

#[test]
fn different_symbol_instances_same_value_are_equal() {
    // Soroban Symbol is interned — same string = same Symbol identity
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    let provided = Symbol::new(&env, "INV_123");
    assert_eq!(matches_required_memo(&env, Some(expected), Some(provided)), Ok(()));
}

#[test]
fn boolean_wrapper_returns_true_for_satisfied_cases() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");

    assert!(memo_binding_satisfied(&env, None, None));
    assert!(memo_binding_satisfied(&env, None, Some(symbol_short!("extra"))));
    assert!(memo_binding_satisfied(&env, Some(expected.clone()), Some(expected)));
}

#[test]
fn storage_encode_decode_roundtrip_with_memo() {
    let env = Env::default();
    let expected = Symbol::new(&env, "INV_123");
    let stored = encode_expected_memo(Some(expected.clone()));
    let decoded = decode_expected_memo(stored);
    assert_eq!(decoded, Some(expected));
}

#[test]
fn storage_encode_decode_roundtrip_without_memo() {
    let stored = encode_expected_memo(None);
    let decoded = decode_expected_memo(stored);
    assert_eq!(decoded, None);
}

#[test]
fn storage_sentinel_is_not_a_valid_user_memo() {
    // The sentinel should not collide with any user-provided memo
    // encode(None) -> sentinel, decode(sentinel) -> None
    let sentinel_decoded = decode_expected_memo(encode_expected_memo(None));
    assert_eq!(sentinel_decoded, None, "sentinel should decode to None");

    // We can't directly test the sentinel value since it's private, but we can
    // verify that a user memo with a different string decodes to Some
    let env = Env::default();
    let user_memo = Symbol::new(&env, "USER_MEMO");
    assert!(decode_expected_memo(user_memo).is_some()); // User memo decodes to Some
}

#[test]
fn multiple_destinations_independent_checks() {
    let env = Env::default();
    let dest_a_memo = Symbol::new(&env, "INV_A");
    let _dest_b_memo = Symbol::new(&env, "INV_B");

    // Destination A requires memo, B doesn't
    assert!(matches_required_memo(&env, Some(dest_a_memo.clone()), Some(dest_a_memo)).is_ok());
    assert!(matches_required_memo(&env, None, Some(symbol_short!("any"))).is_ok());
    assert!(matches_required_memo(&env, None, None).is_ok());
}

#[test]
fn memo_with_special_characters() {
    let env = Env::default();
    let memo = Symbol::new(&env, "INV_123_USD_2024");
    assert!(matches_required_memo(&env, Some(memo.clone()), Some(memo)).is_ok());
}