//! Happy-path tests for `seller-reputation-counter`.
//!
//! These tests cover the successful operation scenarios.

use seller_reputation_counter::{record_success, record_dispute, total_invoices, success_rate, ReputationScore};

#[test]
fn record_success_increments_count() {
    let current: ReputationScore = (0, 0);

    let updated = record_success(current).unwrap();
    assert_eq!(updated, (1, 0));

    let updated = record_success(updated).unwrap();
    assert_eq!(updated, (2, 0));
}

#[test]
fn record_dispute_increments_count() {
    let current: ReputationScore = (0, 0);

    let updated = record_dispute(current).unwrap();
    assert_eq!(updated, (0, 1));

    let updated = record_dispute(updated).unwrap();
    assert_eq!(updated, (0, 2));
}

#[test]
fn success_and_dispute_independent() {
    let current: ReputationScore = (2, 0);

    let updated = record_dispute(current).unwrap();
    assert_eq!(updated, (2, 1));
}

#[test]
fn multiple_sellers_independent() {
    let s1: ReputationScore = (0, 0);
    let s2: ReputationScore = (0, 0);

    let s1 = record_success(s1).unwrap();
    let s1 = record_success(s1).unwrap();
    let s2 = record_dispute(s2).unwrap();

    assert_eq!(s1, (2, 0));
    assert_eq!(s2, (0, 1));
}

#[test]
fn total_invoices_sums_both() {
    let score: ReputationScore = (2, 1);
    assert_eq!(total_invoices(score), 3);
}

#[test]
fn success_rate_calculates_correctly() {
    // No invoices
    assert_eq!(success_rate((0, 0)), None);

    // 1 success, 0 disputes = 100%
    assert_eq!(success_rate((1, 0)), Some(100));

    // 1 success, 1 dispute = 50%
    assert_eq!(success_rate((1, 1)), Some(50));

    // 3 success, 1 dispute = 75%
    assert_eq!(success_rate((3, 1)), Some(75));
}