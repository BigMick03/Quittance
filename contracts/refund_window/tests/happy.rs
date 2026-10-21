use refund_window::refund_allowed;

#[test]
fn refund_inside_the_window_is_allowed() {
    assert!(refund_allowed(100, 110, 20));
}
