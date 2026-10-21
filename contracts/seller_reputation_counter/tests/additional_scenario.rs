//! Additional scenario tests for `seller-reputation-counter`.
//!
//! These tests cover specific scenarios relevant to seller reputation tracking
//! on Stellar, such as marketplace workflows, dispute resolution, and analytics.

use seller_reputation_counter::{record_success, record_dispute, total_invoices, success_rate, ReputationScore};

#[test]
fn marketplace_seller_onboarding() {
    // Scenario: New seller joins marketplace, builds reputation over time
    let mut score: ReputationScore = (0, 0);

    // Week 1: 5 successful invoices
    for _ in 0..5 {
        score = record_success(score).unwrap();
    }
    assert_eq!(score, (5, 0));
    assert_eq!(success_rate(score), Some(100));

    // Week 2: 1 dispute, 3 successes
    score = record_dispute(score).unwrap();
    for _ in 0..3 {
        score = record_success(score).unwrap();
    }
    assert_eq!(score, (8, 1));
    assert_eq!(success_rate(score), Some(88)); // 8/9 = 88%

    // Week 3: 2 more disputes
    score = record_dispute(score).unwrap();
    score = record_dispute(score).unwrap();
    assert_eq!(score, (8, 3));
    assert_eq!(success_rate(score), Some(72)); // 8/11 = 72%
}

#[test]
fn multiple_seller_marketplace() {
    // Scenario: Marketplace with multiple sellers
    let mut s1: ReputationScore = (0, 0);
    let mut s2: ReputationScore = (0, 0);
    let s3: ReputationScore = (0, 0);

    // Seller 1: Excellent (20 success, 0 disputes)
    for _ in 0..20 {
        s1 = record_success(s1).unwrap();
    }

    // Seller 2: Good (15 success, 2 disputes)
    for _ in 0..15 {
        s2 = record_success(s2).unwrap();
    }
    s2 = record_dispute(s2).unwrap();
    s2 = record_dispute(s2).unwrap();

    assert_eq!(s1, (20, 0));
    assert_eq!(s2, (15, 2));
    assert_eq!(s3, (0, 0));

    assert_eq!(success_rate(s1), Some(100));
    assert_eq!(success_rate(s2), Some(88)); // 15/17 = 88%
    assert_eq!(success_rate(s3), None);
}

#[test]
fn dispute_resolution_workflow() {
    // Scenario: Dispute is recorded, then later resolved in seller's favor
    let mut score: ReputationScore = (0, 0);

    // Initial dispute
    score = record_dispute(score).unwrap();
    assert_eq!(score, (0, 1));
    assert_eq!(success_rate(score), Some(0));

    // Dispute resolved in seller's favor - record as success
    score = record_success(score).unwrap();
    assert_eq!(score, (1, 1));
    assert_eq!(success_rate(score), Some(50));
}

#[test]
fn reputation_based_sorting() {
    // Scenario: Frontend wants to sort sellers by reputation
    let mut sellers: Vec<ReputationScore> = vec![(0, 0), (0, 0), (0, 0)];

    // Seller A: 90% success (9/10)
    for _ in 0..9 { sellers[0] = record_success(sellers[0]).unwrap(); }
    sellers[0] = record_dispute(sellers[0]).unwrap();

    // Seller B: 80% success (8/10)
    for _ in 0..8 { sellers[1] = record_success(sellers[1]).unwrap(); }
    for _ in 0..2 { sellers[1] = record_dispute(sellers[1]).unwrap(); }

    // Seller C: 50% success (5/10)
    for _ in 0..5 { sellers[2] = record_success(sellers[2]).unwrap(); }
    for _ in 0..5 { sellers[2] = record_dispute(sellers[2]).unwrap(); }

    // Sort by success rate (descending)
    let mut rates: Vec<(u32, u32)> = Vec::new();
    for score in &sellers {
        let rate = success_rate(*score).unwrap_or(0);
        let total = score.0 + score.1;
        rates.push((rate, total));
    }

    rates.sort_by_key(|a| std::cmp::Reverse(a.0)); // Descending by rate

    // Seller A (90%) first, then B (80%), then C (50%)
    assert_eq!(rates[0].0, 90);
    assert_eq!(rates[1].0, 80);
    assert_eq!(rates[2].0, 50);
}

#[test]
fn idempotent_calculations() {
    // Multiple calculations don't affect input
    let score: ReputationScore = (5, 3);

    for _ in 0..100 {
        assert_eq!(success_rate(score), Some(62)); // 5/8 = 62%
        assert_eq!(total_invoices(score), 8);
    }
}

#[test]
fn key_generation_consistency() {
    // Key generation is consistent
    let key1 = seller_reputation_counter::reputation_key("GABC123");
    let key2 = seller_reputation_counter::reputation_key("GABC123");
    assert_eq!(key1, key2);
    assert_eq!(key1, "REP_GABC123");
}