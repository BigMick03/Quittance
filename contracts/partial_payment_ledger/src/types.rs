/// Why a payment could not be recorded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaymentError {
    /// `total`, `paid_so_far`, or `new_payment` was negative.
    NegativeAmount,
    /// The recorded paid amount was already above the invoice total.
    PaidExceedsTotal,
    /// Applying the new payment would exceed the invoice total.
    Overpayment,
}

/// Paid amount after a successful record, and whether the invoice is settled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaymentProgress {
    /// Invoice total in the smallest asset unit.
    pub total: i128,
    /// Amount recorded so far, never above `total`.
    pub paid_so_far: i128,
}

impl PaymentProgress {
    /// Start an unpaid invoice.
    pub fn new(total: i128) -> Result<Self, PaymentError> {
        if total < 0 {
            return Err(PaymentError::NegativeAmount);
        }
        Ok(Self {
            total,
            paid_so_far: 0,
        })
    }

    /// Apply one payment. Overpayment leaves this value unchanged.
    pub fn record(&mut self, new_payment: i128) -> Result<bool, PaymentError> {
        let (paid, settled) = crate::logic::record_payment(self.total, self.paid_so_far, new_payment)?;
        self.paid_so_far = paid;
        Ok(settled)
    }
}
