//! Staleness check for a stored price rate and timestamp.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod logic;
mod types;

pub use crate::logic::is_fresh;
pub use crate::types::{PriceSnapshot, SnapshotError};
