use grace_period_guard::{is_within_grace, remaining_grace};

#[test]
fn last_second_of_grace_is_still_inside() {
    let due_at = 1_000;
    let grace_secs = 50;
    let paid_at = 1_050;

    assert!(is_within_grace(due_at, grace_secs, paid_at));
    assert_eq!(remaining_grace(due_at, grace_secs, paid_at), 0);
}

#[test]
fn zero_grace_allows_only_the_due_instant() {
    assert!(is_within_grace(10, 0, 10));
    assert!(!is_within_grace(10, 0, 11));
}
