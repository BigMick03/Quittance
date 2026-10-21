//! Core validation logic for batch invoice creation.

use crate::types::{BatchConfig, BatchError, BatchResult};

/// Validate a batch of invoice amounts against a maximum batch size and maximum total amount.
///
/// This is the primary public function. It checks, in order:
/// 1. `max_count` must be greater than zero.
/// 2. `max_total` must be non-negative.
/// 3. The batch must not be empty.
/// 4. The batch length must not exceed `max_count`.
/// 5. No individual amount may be negative.
/// 6. No individual amount may be zero.
/// 7. The sum of all amounts must not exceed `max_total`.
///
/// On the first failure, the corresponding [`BatchError`] is returned.
/// The order of checks is part of the public contract so consumers can rely
/// on deterministic error precedence.
///
/// # Arguments
///
/// * `amounts` — A slice of invoice amounts in stroops (signed 128-bit integers,
///   matching Stellar's on-chain representation).
/// * `max_count` — Maximum number of invoices allowed in the batch.
/// * `max_total` — Maximum sum of all invoice amounts in the batch, in stroops.
///
/// # Returns
///
/// `Ok(())` if the batch passes all validation checks, or the first
/// [`BatchError`] encountered.
///
/// # Examples
///
/// ```
/// use batch_create_guard::validate_batch;
///
/// // Happy path: 3 invoices, total 30 XLM, within limits of 5 invoices and 100 XLM.
/// let amounts = [10_000_000_000i128; 3]; // 10 XLM each in stroops
/// assert!(validate_batch(&amounts, 5, 100_000_000_000).is_ok());
///
/// // Fails: batch size exceeds max_count.
/// let amounts = [1_000_000_000i128; 6];
/// assert_eq!(
///     validate_batch(&amounts, 5, 100_000_000_000),
///     Err(batch_create_guard::BatchError::CountExceeded)
/// );
/// ```
pub fn validate_batch(amounts: &[i128], max_count: u32, max_total: i128) -> BatchResult {
    // 1. Validate max_count
    if max_count == 0 {
        return Err(BatchError::InvalidMaxCount);
    }

    // 2. Validate max_total
    if max_total < 0 {
        return Err(BatchError::InvalidMaxTotal);
    }

    // 3. Empty batch check
    if amounts.is_empty() {
        return Err(BatchError::EmptyBatch);
    }

    // 4. Count check
    if amounts.len() > max_count as usize {
        return Err(BatchError::CountExceeded);
    }

    // 5-7. Iterate amounts: negative check, zero check, and sum for total check
    let mut total: i128 = 0;
    for &amount in amounts {
        if amount < 0 {
            return Err(BatchError::NegativeAmount);
        }
        if amount == 0 {
            return Err(BatchError::ZeroAmount);
        }
        total = total.saturating_add(amount);
    }

    // 7. Total check
    if total > max_total {
        return Err(BatchError::TotalExceeded);
    }

    Ok(())
}

/// Validate a batch using a pre-constructed [`BatchConfig`].
///
/// This is a convenience wrapper that delegates to [`validate_batch`] after
/// extracting the parameters from the config. The config itself is validated
/// at construction time via [`BatchConfig::new`].
pub fn validate_batch_with_config(amounts: &[i128], config: &BatchConfig) -> BatchResult {
    validate_batch(amounts, config.max_count, config.max_total)
}

/// Convenience function returning `true` when [`validate_batch`] returns `Ok(())`.
///
/// Mirrors the `is_valid_destination` convenience-wrapper pattern in
/// `destination_guard`: callers that only need a yes/no answer can skip
/// handling the [`BatchError`] enum.
///
/// # Examples
///
/// ```
/// use batch_create_guard::is_valid_batch;
///
/// let amounts = [10_000_000_000i128; 3];
/// assert!(is_valid_batch(&amounts, 5, 100_000_000_000));
/// assert!(!is_valid_batch(&amounts, 2, 100_000_000_000)); // count exceeded
/// ```
#[must_use = "the validation result is ignored; consider using `validate_batch` if you need to distinguish the error reason"]
pub fn is_valid_batch(amounts: &[i128], max_count: u32, max_total: i128) -> bool {
    validate_batch(amounts, max_count, max_total).is_ok()
}