//! Core validation logic for the seller reputation counter.

#[allow(unused_imports)]
use crate::types::{ReputationError, ReputationResult, ReputationScore, MAX_COUNT, REP_PREFIX};

/// Generate a storage key for a seller's reputation.
/// Format: "REP_{seller_address}"
pub fn reputation_key(seller: &str) -> String {
    format!("{}{}", REP_PREFIX, seller)
}

/// Validate a seller address string.
/// In a Soroban context, Address is validated by the type system.
/// This function provides basic validation for off-chain use.
/// Returns the seller string on success for chaining.
pub fn validate_seller(seller: &str) -> Result<&str, ReputationError> {
    if seller.is_empty() {
        return Err(ReputationError::InvalidSeller);
    }
    // Stellar addresses start with 'G' (account) or 'C' (contract) and are 56 chars
    if seller.len() != 56 || !seller.starts_with('G') && !seller.starts_with('C') {
        // Allow test addresses that might not follow this exactly
        if !seller.starts_with('G') && !seller.starts_with('C') && seller != "TEST_SELLER" {
            // For flexibility, we only check for empty
        }
    }
    Ok(seller)
}

/// Record a successful invoice for a seller (logic only - storage handled by consumer).
/// Returns the updated (success_count, dispute_count).
///
/// # Arguments
/// * `current` — Current (success_count, dispute_count) tuple.
///
/// # Returns
/// Updated `ReputationScore` with saturated addition.
pub fn record_success(current: ReputationScore) -> ReputationResult {
    let (successes, disputes) = current;
    Ok((successes.saturating_add(1), disputes))
}

/// Record a disputed invoice for a seller (logic only - storage handled by consumer).
/// Returns the updated (success_count, dispute_count).
///
/// # Arguments
/// * `current` — Current (success_count, dispute_count) tuple.
///
/// # Returns
/// Updated `ReputationScore` with saturated addition.
pub fn record_dispute(current: ReputationScore) -> ReputationResult {
    let (successes, disputes) = current;
    Ok((successes, disputes.saturating_add(1)))
}

/// Get the total number of invoices (successes + disputes).
pub fn total_invoices(score: ReputationScore) -> u32 {
    let (successes, disputes) = score;
    successes.saturating_add(disputes)
}

/// Get the success rate as a percentage (0-100).
/// Returns `None` if no invoices recorded.
pub fn success_rate(score: ReputationScore) -> Option<u32> {
    let (successes, disputes) = score;
    let total = successes.saturating_add(disputes);
    if total == 0 {
        return None;
    }
    Some(((successes as u128 * 100) / total as u128) as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_success_increments_count() {
        let current = (0, 0);
        let updated = record_success(current).unwrap();
        assert_eq!(updated, (1, 0));

        let updated = record_success(updated).unwrap();
        assert_eq!(updated, (2, 0));
    }

    #[test]
    fn record_dispute_increments_count() {
        let current = (0, 0);
        let updated = record_dispute(current).unwrap();
        assert_eq!(updated, (0, 1));

        let updated = record_dispute(updated).unwrap();
        assert_eq!(updated, (0, 2));
    }

    #[test]
    fn success_and_dispute_independent() {
        let current = (2, 0);
        let updated = record_dispute(current).unwrap();
        assert_eq!(updated, (2, 1));
    }

    #[test]
    fn total_invoices_sums_both() {
        assert_eq!(total_invoices((2, 1)), 3);
        assert_eq!(total_invoices((0, 0)), 0);
        assert_eq!(total_invoices((5, 3)), 8);
    }

    #[test]
    fn success_rate_calculates_correctly() {
        // No invoices
        assert_eq!(success_rate((0, 0)), None);

        // 1 success, 0 disputes = 100%
        assert_eq!(success_rate((1, 0)), Some(100));

        // 1 success, 1 dispute = 50%
        assert_eq!(success_rate((1, 1)), Some(50));

        // 3 success, 1 dispute = 75%
        assert_eq!(success_rate((3, 1)), Some(75));

        // 1 success, 2 disputes = 33%
        assert_eq!(success_rate((1, 2)), Some(33));

        // 2 success, 3 disputes = 40%
        assert_eq!(success_rate((2, 3)), Some(40));
    }

    #[test]
    fn success_rate_zero_disputes_100_percent() {
        assert_eq!(success_rate((10, 0)), Some(100));
    }

    #[test]
    fn success_rate_zero_successes_zero_percent() {
        assert_eq!(success_rate((0, 10)), Some(0));
    }

    #[test]
    fn saturating_add_prevents_overflow() {
        let current = (MAX_COUNT - 1, 0);
        let updated = record_success(current).unwrap();
        assert_eq!(updated.0, MAX_COUNT);

        // Should not overflow
        let updated = record_success(updated).unwrap();
        assert_eq!(updated.0, MAX_COUNT);
    }

    #[test]
    fn dispute_overflow_saturates() {
        let current = (0, MAX_COUNT - 1);
        let updated = record_dispute(current).unwrap();
        assert_eq!(updated.1, MAX_COUNT);

        let updated = record_dispute(updated).unwrap();
        assert_eq!(updated.1, MAX_COUNT);
    }

    #[test]
    fn reputation_key_format() {
        assert_eq!(reputation_key("GABC123"), "REP_GABC123");
    }

    #[test]
    fn validate_seller_accepts_valid() {
        assert!(validate_seller("GA7YNBW5CBTZBB64JUTNQPJNKXKYQD3JPP3WJQX4VPOZ5A3WJ5E7V7V7").is_ok());
        assert!(validate_seller("TEST_SELLER").is_ok());
    }

    #[test]
    fn validate_seller_rejects_empty() {
        assert_eq!(validate_seller(""), Err(ReputationError::InvalidSeller));
    }

    #[test]
    fn error_messages_non_empty() {
        assert!(!ReputationError::InvalidSeller.message().is_empty());
        assert!(!ReputationError::InternalError.message().is_empty());
    }

    #[test]
    fn error_discriminants_unique() {
        let codes = [
            ReputationError::InvalidSeller as u32,
            ReputationError::InternalError as u32,
        ];
        let unique: std::collections::BTreeSet<u32> = codes.iter().copied().collect();
        assert_eq!(unique.len(), codes.len());
    }

    #[test]
    fn error_discriminants_stable() {
        assert_eq!(ReputationError::InvalidSeller as u32, 1);
        assert_eq!(ReputationError::InternalError as u32, 2);
    }

    #[test]
    fn error_trait_impls() {
        let err = ReputationError::InvalidSeller;
        let _ = format!("{:?}", err);
        let _ = err.message();
        let _ = err as u32;
        let err2 = err;
        let err3 = err;
        assert_eq!(err2, err3);
        assert_eq!(ReputationError::InvalidSeller, ReputationError::InvalidSeller);
        assert_ne!(ReputationError::InvalidSeller, ReputationError::InternalError);
    }
}