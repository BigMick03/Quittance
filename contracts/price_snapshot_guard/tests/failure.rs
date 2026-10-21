use price_snapshot_guard::{is_fresh, PriceSnapshot, SnapshotError};

#[test]
fn one_second_past_the_limit_is_stale() {
    assert!(!is_fresh(1_000, 1_061, 60));
}

#[test]
fn clock_before_the_snapshot_is_not_fresh() {
    assert!(!is_fresh(1_000, 999, 60));
}

#[test]
fn non_positive_rate_is_rejected() {
    assert_eq!(
        PriceSnapshot::new(0, 1_000).unwrap_err(),
        SnapshotError::NonPositiveRate
    );
}
