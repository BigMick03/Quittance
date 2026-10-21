//! Additional scenario tests for `payout-memo-binding`.
//!
//! These tests cover specific scenarios relevant to payout memo binding
//! on Stellar, such as contract storage round-trips, multiple destinations,
//! and batch validation workflows.

use payout_memo_binding::{matches_required_memo, memo_binding_satisfied, encode_expected_memo, decode_expected_memo, MemoBindingError};
use soroban_sdk::{Env, Symbol, symbol_short};

#[test]
fn contract_storage_roundtrip_encode_decode() {
    // Test the encode/decode functions without actual contract storage
    // (storage requires a real contract environment)
    let expected = Symbol::new(&Env::default(), "INV_A_123");
    let stored = encode_expected_memo(Some(expected.clone()));
    let decoded = decode_expected_memo(stored);
    assert_eq!(decoded, Some(expected));

    // No memo case
    let stored_none = encode_expected_memo(None);
    let decoded_none = decode_expected_memo(stored_none);
    assert_eq!(decoded_none, None);
}

#[test]
fn multiple_payouts_batch_validation() {
    let env = Env::default();

    // Batch of payouts to different destinations
    let dest_a_memo = Some(Symbol::new(&env, "INV_A"));
    let dest_b_memo: Option<Symbol> = None;
    let dest_c_memo = Some(Symbol::new(&env, "INV_C"));

    // All payments have correct memos
    assert!(matches_required_memo(&env, dest_a_memo.clone(), Some(Symbol::new(&env, "INV_A"))).is_ok());
    assert!(matches_required_memo(&env, dest_b_memo, None).is_ok());
    assert!(matches_required_memo(&env, dest_c_memo, Some(Symbol::new(&env, "INV_C"))).is_ok());

    // One wrong memo in batch
    assert!(matches_required_memo(&env, dest_a_memo, Some(symbol_short!("WRONG"))).is_err());
}

#[test]
fn client_side_prevalidation_before_submitting() {
    let env = Env::default();

    // Client knows the destination's memo requirement
    let dest_requires_memo = Some(Symbol::new(&env, "INV_456"));
    let dest_no_memo: Option<Symbol> = None;

    // User enters memo in UI
    let user_enters_correct = Some(Symbol::new(&env, "INV_456"));
    let user_enters_wrong = Some(symbol_short!("WRONG"));
    let user_enters_none: Option<Symbol> = None;

    // Pre-validation for destination requiring memo
    assert!(memo_binding_satisfied(&env, dest_requires_memo.clone(), user_enters_correct.clone()));
    assert!(!memo_binding_satisfied(&env, dest_requires_memo.clone(), user_enters_wrong.clone()));
    assert!(!memo_binding_satisfied(&env, dest_requires_memo, user_enters_none.clone()));

    // Pre-validation for destination not requiring memo
    assert!(memo_binding_satisfied(&env, dest_no_memo.clone(), user_enters_correct));
    assert!(memo_binding_satisfied(&env, dest_no_memo.clone(), user_enters_wrong));
    assert!(memo_binding_satisfied(&env, dest_no_memo, user_enters_none));
}

#[test]
fn server_side_webhook_validation() {
    // Simulate a webhook receiving a payment notification
    // and validating the memo before crediting the user
    let env = Env::default();

    // Known destination requirements (from database/config)
    let destination_requirements: std::collections::HashMap<String, Option<Symbol>> = [
        ("GABC123...".to_string(), Some(Symbol::new(&env, "INV_789"))),
        ("GDEF456...".to_string(), None),
    ].into_iter().collect();

    // Incoming payment notification
    let incoming_payment = ("GABC123...".to_string(), Some(Symbol::new(&env, "INV_789")));

    if let Some(expected) = destination_requirements.get(&incoming_payment.0) {
        let result = matches_required_memo(&env, expected.clone(), incoming_payment.1);
        assert!(result.is_ok());
    }

    // Payment with wrong memo
    let wrong_payment = ("GABC123...".to_string(), Some(symbol_short!("WRONG")));
    if let Some(expected) = destination_requirements.get(&wrong_payment.0) {
        let result = matches_required_memo(&env, expected.clone(), wrong_payment.1);
        assert_eq!(result, Err(MemoBindingError::MemoMismatch));
    }
}

#[test]
fn memo_requirement_change_detection() {
    // Scenario: destination changes its memo requirement
    // Old requirement: no memo
    // New requirement: memo "INV-NEW"
    let env = Env::default();

    let old_requirement: Option<Symbol> = None;
    let new_requirement = Some(Symbol::new(&env, "INV_NEW"));

    // Payment with no memo - valid under old, invalid under new
    assert!(matches_required_memo(&env, old_requirement.clone(), None).is_ok());
    assert_eq!(
        matches_required_memo(&env, new_requirement.clone(), None),
        Err(MemoBindingError::MissingRequiredMemo)
    );

    // Payment with new memo - invalid under old (extra allowed but not expected), valid under new
    assert!(matches_required_memo(&env, old_requirement, Some(Symbol::new(&env, "INV_NEW"))).is_ok());
    assert!(matches_required_memo(&env, new_requirement, Some(Symbol::new(&env, "INV_NEW"))).is_ok());
}

#[test]
fn idempotent_validation() {
    // Calling validation multiple times with same inputs yields same result
    let env = Env::default();
    let expected = Some(Symbol::new(&env, "INV_123"));
    let provided = Some(Symbol::new(&env, "INV_123"));

    for _ in 0..100 {
        assert!(matches_required_memo(&env, expected.clone(), provided.clone()).is_ok());
        assert!(memo_binding_satisfied(&env, expected.clone(), provided.clone()));
    }
}

#[test]
fn env_independence() {
    // Validation result should be independent of Env instance
    let env1 = Env::default();
    let env2 = Env::default();
    let expected = Symbol::new(&env1, "INV_123");

    let result1 = matches_required_memo(&env1, Some(expected.clone()), Some(expected.clone()));
    let result2 = matches_required_memo(&env2, Some(expected.clone()), Some(expected));

    assert_eq!(result1, result2);
    assert!(result1.is_ok());
}

#[test]
fn cross_destination_isolation() {
    // Validation for one destination doesn't affect another
    let env = Env::default();
    let dest_a_memo = Some(Symbol::new(&env, "INV_A"));
    let dest_b_memo = Some(Symbol::new(&env, "INV_B"));

    // Valid for A
    assert!(matches_required_memo(&env, dest_a_memo.clone(), Some(Symbol::new(&env, "INV_A"))).is_ok());
    // Invalid for B (wrong memo)
    assert!(matches_required_memo(&env, dest_b_memo.clone(), Some(Symbol::new(&env, "INV_A"))).is_err());

    // Valid for B
    assert!(matches_required_memo(&env, dest_b_memo.clone(), Some(Symbol::new(&env, "INV_B"))).is_ok());
    // Invalid for A (wrong memo)
    assert!(matches_required_memo(&env, dest_a_memo, Some(Symbol::new(&env, "INV_B"))).is_err());
}