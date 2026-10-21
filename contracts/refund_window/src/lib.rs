//! Whether a refund is still inside its ledger window.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod logic;
mod types;

pub use crate::logic::refund_allowed;
pub use crate::types::RefundWindow;
