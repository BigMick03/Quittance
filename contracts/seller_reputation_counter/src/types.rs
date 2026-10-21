//! Type definitions for the seller-reputation-counter crate.

/// Error codes returned by the seller reputation counter.
///
/// Each variant carries a stable `#[repr(u32)]` discriminant so off-chain
/// tooling can match on the numeric code without depending on the Rust
/// identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ReputationError {
    /// The seller address is invalid.
    InvalidSeller = 1,
    /// Internal error: unexpected state (e.g., overflow).
    InternalError = 2,
}

impl ReputationError {
    /// Return a stable English message describing the error.
    pub fn message(&self) -> &'static str {
        match self {
            ReputationError::InvalidSeller => "Seller address is invalid.",
            ReputationError::InternalError => "Internal error during reputation operation.",
        }
    }
}

/// Result type for reputation operations.
pub type ReputationResult = Result<ReputationScore, ReputationError>;

/// A seller's reputation score: (success_count, dispute_count).
pub type ReputationScore = (u32, u32);

/// Maximum count value to prevent overflow (u32::MAX).
pub const MAX_COUNT: u32 = u32::MAX;

/// Storage key prefix for seller reputation entries.
pub(crate) const REP_PREFIX: &str = "REP_";