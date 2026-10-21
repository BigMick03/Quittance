//! Edge and boundary case tests for `batch-create-guard`.
//!
//! These tests cover boundary conditions, off-by-one scenarios, and
//! maximum-value edge cases.

use batch_create_guard::{validate_batch, is_valid_batch, BatchConfig, BatchError};

const ONE_XLM_STROOPS: i128 = 10_000_000;
const TEN_XLM_STROOPS: i128 = 100_000_000;
const HUNDRED_XLM_STROOPS: i128 = 1_000_000_000;

#[test]
fn empty_batch_rejected() {
    let amounts: [i128; 0] = [];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Err(BatchError::EmptyBatch));
    assert!(!is_valid_batch(&amounts, 5, HUNDRED_XLM_STROOPS));
}

#[test]
fn count_exceeded_by_one() {
    let amounts = [ONE_XLM_STROOPS; 6];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Err(BatchError::CountExceeded));
    assert!(!is_valid_batch(&amounts, 5, HUNDRED_XLM_STROOPS));
}

#[test]
fn total_exceeded_by_one() {
    // 11 invoices of 10 XLM = 110 XLM, limit is 100 XLM
    let amounts = [TEN_XLM_STROOPS; 11];
    assert_eq!(validate_batch(&amounts, 20, HUNDRED_XLM_STROOPS), Err(BatchError::TotalExceeded));
    assert!(!is_valid_batch(&amounts, 20, HUNDRED_XLM_STROOPS));
}

#[test]
fn max_count_of_one_accepts_single_invoice() {
    let amounts = [TEN_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 1, HUNDRED_XLM_STROOPS), Ok(()));
}

#[test]
fn max_count_of_one_rejects_two_invoices() {
    let amounts = [TEN_XLM_STROOPS, TEN_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 1, HUNDRED_XLM_STROOPS), Err(BatchError::CountExceeded));
}

#[test]
fn max_total_of_zero_rejects_any_positive_amount() {
    let amounts = [ONE_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 5, 0), Err(BatchError::TotalExceeded));
}

#[test]
fn max_total_of_zero_accepts_empty_batch_but_empty_is_rejected_first() {
    let amounts: [i128; 0] = [];
    // Empty batch check comes before total check
    assert_eq!(validate_batch(&amounts, 5, 0), Err(BatchError::EmptyBatch));
}

#[test]
fn max_total_at_i128_max_accepts_any_realistic_batch() {
    let amounts = [HUNDRED_XLM_STROOPS; 100];
    assert_eq!(validate_batch(&amounts, 200, i128::MAX), Ok(()));
}

#[test]
fn count_at_usize_max_is_handled() {
    // u32::MAX would overflow usize on 32-bit, but we test a large value
    // that fits in usize. The check is amounts.len() > max_count as usize.
    let max_count = u32::MAX / 2; // Large but safe for usize conversion
    let amounts = vec![ONE_XLM_STROOPS; 1000]; // Much smaller than max_count
    assert_eq!(validate_batch(&amounts, max_count, HUNDRED_XLM_STROOPS * 1000), Ok(()));
}

#[test]
fn off_by_one_count_boundary() {
    // At boundary: exactly max_count should pass
    let amounts = [ONE_XLM_STROOPS; 5];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Ok(()));
    // One over: should fail
    let amounts_over = [ONE_XLM_STROOPS; 6];
    assert_eq!(validate_batch(&amounts_over, 5, HUNDRED_XLM_STROOPS), Err(BatchError::CountExceeded));
}

#[test]
fn off_by_one_total_boundary() {
    // At boundary: exactly max_total should pass
    let amounts = [TEN_XLM_STROOPS; 10]; // 100 XLM
    assert_eq!(validate_batch(&amounts, 20, HUNDRED_XLM_STROOPS), Ok(()));
    // One over: should fail
    let amounts_over = [TEN_XLM_STROOPS; 11]; // 110 XLM
    assert_eq!(validate_batch(&amounts_over, 20, HUNDRED_XLM_STROOPS), Err(BatchError::TotalExceeded));
}

#[test]
fn single_invoice_at_max_total() {
    // One invoice equal to max_total should pass
    let amounts = [HUNDRED_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Ok(()));
}

#[test]
fn single_invoice_exceeding_max_total() {
    let amounts = [HUNDRED_XLM_STROOPS + 1];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Err(BatchError::TotalExceeded));
}

#[test]
fn negative_max_count_rejected_at_config_construction() {
    // This is tested via the constructor validation
    let result = BatchConfig::new(0, HUNDRED_XLM_STROOPS);
    assert_eq!(result, Err(BatchError::InvalidMaxCount));
}

#[test]
fn negative_max_total_rejected_at_config_construction() {
    let result = BatchConfig::new(5, -1);
    assert_eq!(result, Err(BatchError::InvalidMaxTotal));
}

#[test]
fn large_batch_with_small_amounts() {
    // 10,000 invoices of 1 stroop each
    let amounts = vec![1i128; 10_000];
    let max_total = 10_000i128;
    assert_eq!(validate_batch(&amounts, 20_000, max_total), Ok(()));
}

#[test]
fn single_stroop_amounts() {
    // Minimum non-zero amount on Stellar is 1 stroop
    let amounts = [1i128; 5];
    assert_eq!(validate_batch(&amounts, 10, 100), Ok(()));
}