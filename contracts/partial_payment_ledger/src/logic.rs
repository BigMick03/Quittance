use crate::types::PaymentError;

/// Add `new_payment` to `paid_so_far` without going past `total`.
///
/// Returns `(new_paid_so_far, is_fully_paid)`. `is_fully_paid` is true only
/// when the new total equals `total`. A payment that would exceed `total` is
/// rejected and is not capped.
pub fn record_payment(
    total: i128,
    paid_so_far: i128,
    new_payment: i128,
) -> Result<(i128, bool), PaymentError> {
    if total < 0 || paid_so_far < 0 || new_payment < 0 {
        return Err(PaymentError::NegativeAmount);
    }
    if paid_so_far > total {
        return Err(PaymentError::PaidExceedsTotal);
    }

    let new_paid = paid_so_far.checked_add(new_payment).ok_or(PaymentError::Overpayment)?;
    if new_paid > total {
        return Err(PaymentError::Overpayment);
    }

    Ok((new_paid, new_paid == total))
}
