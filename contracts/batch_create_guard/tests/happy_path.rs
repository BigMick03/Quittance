//! Happy-path tests for `batch-create-guard`.
//!
//! These tests cover the successful validation scenarios where all
//! constraints are satisfied.

use batch_create_guard::{validate_batch, validate_batch_with_config, is_valid_batch, BatchConfig};

const ONE_XLM_STROOPS: i128 = 10_000_000;
const TEN_XLM_STROOPS: i128 = 100_000_000;
const HUNDRED_XLM_STROOPS: i128 = 1_000_000_000;

#[test]
fn single_invoice_within_limits() {
    let amounts = [TEN_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Ok(()));
}

#[test]
fn multiple_invoices_within_limits() {
    let amounts = [TEN_XLM_STROOPS, TEN_XLM_STROOPS, TEN_XLM_STROOPS];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Ok(()));
}

#[test]
fn exactly_at_max_count() {
    let amounts = [ONE_XLM_STROOPS; 5];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Ok(()));
}

#[test]
fn exactly_at_max_total() {
    let amounts = [TEN_XLM_STROOPS; 10]; // 100 XLM total
    assert_eq!(validate_batch(&amounts, 20, HUNDRED_XLM_STROOPS), Ok(()));
}

#[test]
fn both_count_and_total_at_maximum() {
    // 5 invoices of 20 XLM each = 100 XLM total
    // 20 XLM = 20 * 10_000_000 = 200_000_000 stroops
    let amounts = [200_000_000i128; 5];
    assert_eq!(validate_batch(&amounts, 5, HUNDRED_XLM_STROOPS), Ok(()));
}

#[test]
fn boolean_wrapper_returns_true_for_valid_batch() {
    let amounts = [TEN_XLM_STROOPS, TEN_XLM_STROOPS, TEN_XLM_STROOPS];
    assert!(is_valid_batch(&amounts, 5, HUNDRED_XLM_STROOPS));
}

#[test]
fn config_object_works_for_repeated_validation() {
    let config = BatchConfig::new(5, HUNDRED_XLM_STROOPS).unwrap();
    let amounts1 = [TEN_XLM_STROOPS; 3];
    let amounts2 = [TEN_XLM_STROOPS; 2];

    assert_eq!(validate_batch_with_config(&amounts1, &config), Ok(()));
    assert_eq!(validate_batch_with_config(&amounts2, &config), Ok(()));
}

#[test]
fn large_amounts_within_i128_range() {
    // Test with large but realistic invoice amounts (up to ~170 billion XLM)
    let large_amount: i128 = 1_000_000_000_000_000; // ~100 million XLM
    let amounts = [large_amount, large_amount];
    let max_total = large_amount * 2;

    assert_eq!(validate_batch(&amounts, 10, max_total), Ok(()));
}

#[test]
fn mixed_amounts_within_limits() {
    let amounts = [
        50_000_000i128,    // 5 XLM
        150_000_000i128,   // 15 XLM
        250_000_000i128,   // 25 XLM
        50_000_000i128,    // 5 XLM
    ]; // Total: 50 XLM
    assert_eq!(validate_batch(&amounts, 10, HUNDRED_XLM_STROOPS), Ok(()));
}

#[test]
fn config_construction_succeeds_with_valid_params() {
    let config = BatchConfig::new(100, 1_000_000_000_000).unwrap();
    assert_eq!(config.max_count, 100);
    assert_eq!(config.max_total, 1_000_000_000_000);
}