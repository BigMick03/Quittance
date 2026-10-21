use partial_payment_ledger::record_payment;

#[test]
fn partial_payment_updates_the_running_total() {
    let (paid, settled) = record_payment(1_000, 200, 300).unwrap();
    assert_eq!(paid, 500);
    assert!(!settled);
}
