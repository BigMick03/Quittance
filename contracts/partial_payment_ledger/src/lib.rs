//! Running total for an invoice paid in parts.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod logic;
mod types;

pub use crate::logic::record_payment;
pub use crate::types::{PaymentError, PaymentProgress};
