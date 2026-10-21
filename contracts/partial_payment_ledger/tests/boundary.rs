use partial_payment_ledger::record_payment;

#[test]
fn exact_remaining_amount_settles_the_invoice() {
    let (paid, settled) = record_payment(1_000, 400, 600).unwrap();
    assert_eq!(paid, 1_000);
    assert!(settled);
}

#[test]
fn zero_payment_on_a_settled_invoice_stays_settled() {
    let (paid, settled) = record_payment(10, 10, 0).unwrap();
    assert_eq!(paid, 10);
    assert!(settled);
}
