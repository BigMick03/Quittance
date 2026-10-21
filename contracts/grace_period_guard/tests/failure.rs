use grace_period_guard::{is_within_grace, remaining_grace};

#[test]
fn one_second_after_grace_is_late() {
    let due_at = 1_000;
    let grace_secs = 50;

    assert!(!is_within_grace(due_at, grace_secs, 1_051));
    assert_eq!(remaining_grace(due_at, grace_secs, 1_051), -1);
}
