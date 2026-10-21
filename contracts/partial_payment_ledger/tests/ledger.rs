use partial_payment_ledger::PaymentProgress;

#[test]
fn progress_keeps_the_previous_total_when_a_payment_is_too_large() {
    let mut progress = PaymentProgress::new(100).unwrap();
    progress.record(40).unwrap();

    let error = progress.record(70).unwrap_err();

    assert_eq!(error, partial_payment_ledger::PaymentError::Overpayment);
    assert_eq!(progress.paid_so_far, 40);
}
