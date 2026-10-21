//! Failure and error case tests for `batch-create-guard`.
//!
//! These tests cover all the ways a batch can fail validation, ensuring
//! each error variant is triggered correctly and the detection order is
//! respected.

use batch_create_guard::{validate_batch, is_valid_batch, BatchConfig, BatchError};

const TEN_XLM_STROOPS: i128 = 100_000_000;
const HUNDRED_XLM_STROOPS: i128 = 1_000_000_000;

#[test]
fn negative_amount_rejected() {
    let amounts = [TEN_XLM_STROOPS, -TEN_XLM_STROOPS, TEN_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Err(BatchError::NegativeAmount));
    assert!(!is_valid_batch(&amounts, 5, HUNDRED_XLM_STROOPS));
}

#[test]
fn zero_amount_rejected() {
    let amounts = [TEN_XLM_STROOPS, 0, TEN_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Err(BatchError::ZeroAmount));
    assert!(!is_valid_batch(&amounts, 5, HUNDRED_XLM_STROOPS));
}

#[test]
fn count_exceeded_precedes_negative_amount_in_order() {
    // Count check comes before negative amount check in the loop
    let amounts = [-1i128; 10]; // 10 negative amounts, max_count is 5
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Err(BatchError::CountExceeded));
}

#[test]
fn zero_amount_precedes_total_check_in_order() {
    // Even though total would be exceeded, zero amount check comes first
    let amounts = [0i128; 10]; // 10 zero amounts, sum would be 0 but zero check fires first
    assert_eq!(validate_batch(&amounts, 20, HUNDRED_XLM_STROOPS), Err(BatchError::ZeroAmount));
}

#[test]
fn negative_amount_precedes_total_check_in_order() {
    // Negative amount detected before total would be computed
    let amounts = [-TEN_XLM_STROOPS; 10];
    assert_eq!(validate_batch(&amounts, 20, HUNDRED_XLM_STROOPS), Err(BatchError::NegativeAmount));
}

#[test]
fn count_exceeded_precedes_amount_checks() {
    // Count check comes before negative/zero amount checks
    let amounts_exceed_count = [TEN_XLM_STROOPS; 6]; // 6 items, max_count=5
    assert_eq!(validate_batch(&amounts_exceed_count, 5, HUNDRED_XLM_STROOPS), Err(BatchError::CountExceeded));
}

#[test]
fn empty_batch_precedes_amount_checks() {
    // Empty batch is checked after parameter validation but before amount checks
    let amounts: [i128; 0] = [];
    // With valid params, empty batch should be detected
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Err(BatchError::EmptyBatch));
    // But invalid params are caught first
    assert_eq!(validate_batch(&amounts, 0, HUNDRED_XLM_STROOPS), Err(BatchError::InvalidMaxCount));
    assert_eq!(validate_batch(&amounts, 5, -1), Err(BatchError::InvalidMaxTotal));
}

#[test]
fn invalid_max_count_rejected_before_empty_batch() {
    // max_count=0 is invalid and checked first
    let amounts: [i128; 0] = [];
    // Wait - the code checks max_count first, then max_total, then empty
    // So with max_count=0, we should get InvalidMaxCount, not EmptyBatch
    assert_eq!(validate_batch(&amounts, 0, HUNDRED_XLM_STROOPS), Err(BatchError::InvalidMaxCount));
}

#[test]
fn invalid_max_total_rejected_after_max_count_before_empty() {
    let amounts: [i128; 0] = [];
    assert_eq!(validate_batch(&amounts, 5, -1), Err(BatchError::InvalidMaxTotal));
}

#[test]
fn max_count_zero_rejects_non_empty_batch() {
    let amounts = [TEN_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 0, HUNDRED_XLM_STROOPS), Err(BatchError::InvalidMaxCount));
}

#[test]
fn max_total_negative_rejects_non_empty_batch() {
    let amounts = [TEN_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 5, -1), Err(BatchError::InvalidMaxTotal));
}

#[test]
fn all_error_variants_have_non_empty_messages() {
    for err in [
        BatchError::EmptyBatch,
        BatchError::CountExceeded,
        BatchError::TotalExceeded,
        BatchError::NegativeAmount,
        BatchError::ZeroAmount,
        BatchError::InvalidMaxCount,
        BatchError::InvalidMaxTotal,
    ] {
        assert!(!err.message().is_empty(), "Error {:?} message is empty", err);
    }
}

#[test]
fn error_discriminants_are_unique() {
    let codes = [
        BatchError::EmptyBatch as u32,
        BatchError::CountExceeded as u32,
        BatchError::TotalExceeded as u32,
        BatchError::NegativeAmount as u32,
        BatchError::ZeroAmount as u32,
        BatchError::InvalidMaxCount as u32,
        BatchError::InvalidMaxTotal as u32,
    ];
    let unique: std::collections::BTreeSet<u32> = codes.iter().copied().collect();
    assert_eq!(unique.len(), codes.len(), "discriminants must not collide");
}

#[test]
fn error_discriminants_match_expected_values() {
    assert_eq!(BatchError::EmptyBatch as u32, 1);
    assert_eq!(BatchError::CountExceeded as u32, 2);
    assert_eq!(BatchError::TotalExceeded as u32, 3);
    assert_eq!(BatchError::NegativeAmount as u32, 4);
    assert_eq!(BatchError::ZeroAmount as u32, 5);
    assert_eq!(BatchError::InvalidMaxCount as u32, 6);
    assert_eq!(BatchError::InvalidMaxTotal as u32, 7);
}

#[test]
fn config_construction_rejects_zero_max_count() {
    assert_eq!(BatchConfig::new(0, HUNDRED_XLM_STROOPS), Err(BatchError::InvalidMaxCount));
}

#[test]
fn config_construction_rejects_negative_max_total() {
    assert_eq!(BatchConfig::new(5, -1), Err(BatchError::InvalidMaxTotal));
}

#[test]
fn config_construction_succeeds_with_valid_values() {
    let config = BatchConfig::new(10, 1000).unwrap();
    assert_eq!(config.max_count, 10);
    assert_eq!(config.max_total, 1000);
}

#[test]
fn boolean_wrapper_returns_false_for_all_error_types() {
    // Empty batch
    assert!(!is_valid_batch(&[], 5, HUNDRED_XLM_STROOPS));
    // Count exceeded
    assert!(!is_valid_batch(&[TEN_XLM_STROOPS; 6], 5, HUNDRED_XLM_STROOPS));
    // Total exceeded
    assert!(!is_valid_batch(&[TEN_XLM_STROOPS; 11], 20, HUNDRED_XLM_STROOPS));
    // Negative amount
    assert!(!is_valid_batch(&[-1], 5, HUNDRED_XLM_STROOPS));
    // Zero amount
    assert!(!is_valid_batch(&[0], 5, HUNDRED_XLM_STROOPS));
    // Invalid max_count
    assert!(!is_valid_batch(&[TEN_XLM_STROOPS], 0, HUNDRED_XLM_STROOPS));
    // Invalid max_total
    assert!(!is_valid_batch(&[TEN_XLM_STROOPS], 5, -1));
}