use refund_window::refund_allowed;

#[test]
fn last_ledger_of_the_window_is_allowed() {
    assert!(refund_allowed(100, 120, 20));
}

#[test]
fn zero_window_allows_only_the_payment_ledger() {
    assert!(refund_allowed(100, 100, 0));
    assert!(!refund_allowed(100, 101, 0));
}
