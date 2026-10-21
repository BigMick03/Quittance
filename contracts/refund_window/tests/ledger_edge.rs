use refund_window::RefundWindow;

#[test]
fn window_near_the_last_ledger_does_not_wrap() {
    let window = RefundWindow::new(u32::MAX - 2, 5);
    assert!(window.allows(u32::MAX));
    assert!(!window.allows(u32::MAX - 3));
}
