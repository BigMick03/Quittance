/// Why a price snapshot could not be stored.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotError {
    /// Rate must be a positive quote in the smallest asset unit.
    NonPositiveRate,
}

/// A price quote and the time it was observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PriceSnapshot {
    /// Positive rate in the smallest asset unit.
    pub rate: i128,
    /// Unix time when `rate` was observed.
    pub snapshot_at: u64,
}

impl PriceSnapshot {
    /// Store a positive rate taken at `snapshot_at`.
    pub fn new(rate: i128, snapshot_at: u64) -> Result<Self, SnapshotError> {
        if rate <= 0 {
            return Err(SnapshotError::NonPositiveRate);
        }
        Ok(Self { rate, snapshot_at })
    }

    /// Whether this quote is still fresh at `now`.
    pub fn is_fresh(self, now: u64, max_age_secs: u64) -> bool {
        crate::logic::is_fresh(self.snapshot_at, now, max_age_secs)
    }
}
