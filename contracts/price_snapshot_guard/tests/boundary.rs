use price_snapshot_guard::is_fresh;

#[test]
fn age_equal_to_the_limit_is_still_fresh() {
    assert!(is_fresh(1_000, 1_060, 60));
}

#[test]
fn zero_max_age_allows_only_the_same_instant() {
    assert!(is_fresh(1_000, 1_000, 0));
    assert!(!is_fresh(1_000, 1_001, 0));
}
