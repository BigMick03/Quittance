use partial_payment_ledger::{record_payment, PaymentError};

#[test]
fn overpayment_is_rejected() {
    let error = record_payment(1_000, 900, 200).unwrap_err();
    assert_eq!(error, PaymentError::Overpayment);
}

#[test]
fn negative_payment_is_rejected() {
    let error = record_payment(1_000, 0, -1).unwrap_err();
    assert_eq!(error, PaymentError::NegativeAmount);
}
