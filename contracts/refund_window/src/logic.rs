/// Whether a refund at `current_ledger` is still allowed.
///
/// Allowed when `current_ledger` is at least `paid_ledger` and the elapsed
/// ledger count is less than or equal to `window`. A current ledger before
/// the payment ledger is not allowed. A window of `0` allows only the payment
/// ledger itself.
pub fn refund_allowed(paid_ledger: u32, current_ledger: u32, window: u32) -> bool {
    match current_ledger.checked_sub(paid_ledger) {
        Some(elapsed) => elapsed <= window,
        None => false,
    }
}
