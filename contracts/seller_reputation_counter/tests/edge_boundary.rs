//! Edge and boundary case tests for `seller-reputation-counter`.

use seller_reputation_counter::{total_invoices, success_rate, MAX_COUNT, ReputationScore};

#[test]
fn zero_counts_initially() {
    let _score: (u32, u32) = (0, 0);
    assert_eq!(total_invoices((0, 0)), 0);
    assert_eq!(success_rate((0, 0)), None);
}

#[test]
fn zero_disputes_100_percent_success() {
    let mut score: (u32, u32) = (0, 0);
    for _ in 0..10 {
        score = seller_reputation_counter::record_success(score).unwrap();
    }
    assert_eq!(success_rate(score), Some(100));
}

#[test]
fn zero_successes_zero_percent() {
    let mut score: (u32, u32) = (0, 0);
    for _ in 0..10 {
        score = seller_reputation_counter::record_dispute(score).unwrap();
    }
    assert_eq!(success_rate(score), Some(0));
}

#[test]
fn success_rate_integer_division_truncates() {
    // 1 success, 2 disputes = 33.33...% -> 33%
    let mut score: (u32, u32) = (0, 0);
    score = seller_reputation_counter::record_success(score).unwrap();
    score = seller_reputation_counter::record_dispute(score).unwrap();
    score = seller_reputation_counter::record_dispute(score).unwrap();
    assert_eq!(success_rate(score), Some(33));

    // 2 success, 3 disputes = 40%
    let mut score: (u32, u32) = (0, 0);
    score = seller_reputation_counter::record_success(score).unwrap();
    score = seller_reputation_counter::record_success(score).unwrap();
    for _ in 0..3 {
        score = seller_reputation_counter::record_dispute(score).unwrap();
    }
    assert_eq!(success_rate(score), Some(40));
}

#[test]
fn large_count_values() {
    let mut score: ReputationScore = (0, 0);
    for _ in 0..1000 {
        score = seller_reputation_counter::record_success(score).unwrap();
    }
    assert_eq!(score, (1000, 0));
    assert_eq!(total_invoices(score), 1000);
    assert_eq!(success_rate(score), Some(100));
}

#[test]
fn mixed_large_counts() {
    let mut score: ReputationScore = (0, 0);
    for _ in 0..5000 {
        score = seller_reputation_counter::record_success(score).unwrap();
    }
    for _ in 0..5000 {
        score = seller_reputation_counter::record_dispute(score).unwrap();
    }
    assert_eq!(score, (5000, 5000));
    assert_eq!(total_invoices(score), 10000);
    assert_eq!(success_rate(score), Some(50));
}

#[test]
fn saturating_at_max_count() {
    // Test that we can reach MAX_COUNT
    let mut score: ReputationScore = (MAX_COUNT - 1, 0);
    score = seller_reputation_counter::record_success(score).unwrap();
    assert_eq!(score.0, MAX_COUNT);

    // Additional increments should saturate
    score = seller_reputation_counter::record_success(score).unwrap();
    assert_eq!(score.0, MAX_COUNT);
}