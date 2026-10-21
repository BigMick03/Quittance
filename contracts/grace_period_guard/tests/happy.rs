use grace_period_guard::{is_within_grace, remaining_grace};

#[test]
fn payment_during_grace_is_on_time() {
    let due_at = 1_700_000_000;
    let grace_secs = 86_400;
    let paid_at = due_at + 3_600;

    assert!(is_within_grace(due_at, grace_secs, paid_at));
    assert_eq!(remaining_grace(due_at, grace_secs, paid_at), 82_800);
}
