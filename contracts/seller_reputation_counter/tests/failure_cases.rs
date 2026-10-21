//! Failure and error case tests for `seller-reputation-counter`.

use seller_reputation_counter::{ReputationError, ReputationScore, MAX_COUNT};

#[test]
fn saturating_add_prevents_overflow() {
    let score: ReputationScore = (MAX_COUNT, 0);
    let result = seller_reputation_counter::record_success(score);
    assert_eq!(result.unwrap().0, MAX_COUNT);
}

#[test]
fn dispute_overflow_saturates() {
    let score: ReputationScore = (0, MAX_COUNT);
    let result = seller_reputation_counter::record_dispute(score);
    assert_eq!(result.unwrap().1, MAX_COUNT);
}

#[test]
fn all_error_variants_have_non_empty_messages() {
    for err in [
        ReputationError::InvalidSeller,
        ReputationError::InternalError,
    ] {
        assert!(!err.message().is_empty(), "Error {:?} message is empty", err);
    }
}

#[test]
fn error_discriminants_are_unique() {
    let codes = [
        ReputationError::InvalidSeller as u32,
        ReputationError::InternalError as u32,
    ];
    let unique: std::collections::BTreeSet<u32> = codes.iter().copied().collect();
    assert_eq!(unique.len(), codes.len());
}

#[test]
fn error_discriminants_match_expected_values() {
    assert_eq!(ReputationError::InvalidSeller as u32, 1);
    assert_eq!(ReputationError::InternalError as u32, 2);
}

#[test]
fn error_trait_implementations() {
    let err = ReputationError::InvalidSeller;
    let _ = format!("{:?}", err);
    let _ = err.message();
    let _ = err as u32;
    let err2 = err;
    let err3 = err;
    assert_eq!(err2, err3);
    assert_eq!(ReputationError::InvalidSeller, ReputationError::InvalidSeller);
    assert_ne!(ReputationError::InvalidSeller, ReputationError::InternalError);
}