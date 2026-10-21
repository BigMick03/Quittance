use crate::types::GraceWindow;

/// Whether `paid_at` is at or before `due_at + grace_secs`.
///
/// A payment exactly at the due time is inside the window. A payment exactly
/// `grace_secs` after the due time is still inside. One second later is late.
/// The sum is computed in `i128`, so a `u64` overflow does not wrap the deadline.
pub fn is_within_grace(due_at: u64, grace_secs: u64, paid_at: u64) -> bool {
    GraceWindow { due_at, grace_secs }.deadline() >= paid_at as i128
}

/// Seconds left until the grace deadline, negative when `paid_at` is late.
///
/// The result saturates at `i64::MAX` or `i64::MIN` if the difference does not
/// fit in `i64`.
pub fn remaining_grace(due_at: u64, grace_secs: u64, paid_at: u64) -> i64 {
    let remaining = GraceWindow { due_at, grace_secs }.deadline() - paid_at as i128;
    if remaining > i64::MAX as i128 {
        i64::MAX
    } else if remaining < i64::MIN as i128 {
        i64::MIN
    } else {
        remaining as i64
    }
}

impl GraceWindow {
    /// Absolute deadline in seconds, computed without wrapping.
    pub(crate) fn deadline(self) -> i128 {
        self.due_at as i128 + self.grace_secs as i128
    }
}
