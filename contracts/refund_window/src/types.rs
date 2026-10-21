/// Ledger range during which a paid invoice can still be refunded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RefundWindow {
    /// Ledger sequence that recorded the payment.
    pub paid_ledger: u32,
    /// How many ledgers after `paid_ledger` a refund is still allowed.
    pub window: u32,
}

impl RefundWindow {
    /// Build a window from the payment ledger and its length.
    pub fn new(paid_ledger: u32, window: u32) -> Self {
        Self { paid_ledger, window }
    }

    /// Whether `current_ledger` is still inside this window.
    pub fn allows(self, current_ledger: u32) -> bool {
        crate::logic::refund_allowed(self.paid_ledger, current_ledger, self.window)
    }
}
