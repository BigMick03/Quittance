/// Inclusive payment window that starts at `due_at` and lasts `grace_secs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GraceWindow {
    /// Unix time when the invoice became due.
    pub due_at: u64,
    /// Extra seconds after `due_at` that still count as on time.
    pub grace_secs: u64,
}

impl GraceWindow {
    /// Build a window from a due time and a grace length.
    pub fn new(due_at: u64, grace_secs: u64) -> Self {
        Self { due_at, grace_secs }
    }

    /// Whether `paid_at` is on time, including the last second of grace.
    pub fn contains(self, paid_at: u64) -> bool {
        crate::logic::is_within_grace(self.due_at, self.grace_secs, paid_at)
    }
}
