use price_snapshot_guard::{is_fresh, PriceSnapshot};

#[test]
fn recent_snapshot_is_fresh() {
    let snapshot = PriceSnapshot::new(125_000, 1_700_000_000).unwrap();
    assert!(is_fresh(snapshot.snapshot_at, 1_700_000_030, 60));
    assert!(snapshot.is_fresh(1_700_000_030, 60));
}
