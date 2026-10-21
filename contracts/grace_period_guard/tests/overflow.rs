use grace_period_guard::{remaining_grace, GraceWindow};

#[test]
fn near_max_timestamp_does_not_wrap_the_deadline() {
    let due_at = u64::MAX - 5;
    let window = GraceWindow::new(due_at, 10);

    assert!(window.contains(u64::MAX));
    assert!(remaining_grace(due_at, 10, u64::MAX) > 0);
}
