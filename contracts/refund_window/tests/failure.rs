use refund_window::refund_allowed;

#[test]
fn ledger_before_the_payment_is_not_allowed() {
    assert!(!refund_allowed(100, 99, 20));
}

#[test]
fn one_ledger_past_the_window_is_not_allowed() {
    assert!(!refund_allowed(100, 121, 20));
}
