//! Type definitions for the batch-create-guard crate.

/// Error codes returned by [`validate_batch`].
///
/// Each variant carries a stable `#[repr(u32)]` discriminant so off-chain
/// tooling can match on the numeric code without depending on the Rust
/// identifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum BatchError {
    /// The batch contains no amounts (empty vector).
    EmptyBatch = 1,
    /// The number of invoices in the batch exceeds `max_count`.
    CountExceeded = 2,
    /// The sum of all amounts exceeds `max_total`.
    TotalExceeded = 3,
    /// An individual amount in the batch is negative.
    NegativeAmount = 4,
    /// An individual amount in the batch is zero.
    ZeroAmount = 5,
    /// The `max_count` parameter is zero.
    InvalidMaxCount = 6,
    /// The `max_total` parameter is negative.
    InvalidMaxTotal = 7,
}

impl BatchError {
    /// Return a stable English message describing the error.
    pub fn message(&self) -> &'static str {
        match self {
            BatchError::EmptyBatch => "Batch contains no invoice amounts.",
            BatchError::CountExceeded => "Batch size exceeds the configured maximum count.",
            BatchError::TotalExceeded => "Batch total exceeds the configured maximum total amount.",
            BatchError::NegativeAmount => "Batch contains a negative amount.",
            BatchError::ZeroAmount => "Batch contains a zero amount.",
            BatchError::InvalidMaxCount => "Maximum count must be greater than zero.",
            BatchError::InvalidMaxTotal => "Maximum total must be non-negative.",
        }
    }
}

/// Result type for batch validation.
pub type BatchResult = Result<(), BatchError>;

/// Configuration for batch validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatchConfig {
    /// Maximum number of invoices allowed in a single batch.
    pub max_count: u32,
    /// Maximum total amount (sum of all invoice amounts) allowed in a batch, in stroops.
    pub max_total: i128,
}

impl BatchConfig {
    /// Create a new `BatchConfig` with validation of the parameters.
    pub fn new(max_count: u32, max_total: i128) -> Result<Self, BatchError> {
        if max_count == 0 {
            return Err(BatchError::InvalidMaxCount);
        }
        if max_total < 0 {
            return Err(BatchError::InvalidMaxTotal);
        }
        Ok(Self { max_count, max_total })
    }
}