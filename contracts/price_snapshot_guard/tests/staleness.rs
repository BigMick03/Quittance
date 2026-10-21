use price_snapshot_guard::is_fresh;

#[test]
fn timestamp_near_u64_max_does_not_wrap() {
    assert!(is_fresh(u64::MAX - 5, u64::MAX, 5));
    assert!(!is_fresh(u64::MAX - 5, u64::MAX - 6, 10));
}
