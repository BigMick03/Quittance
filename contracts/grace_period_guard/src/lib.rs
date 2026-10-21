//! Grace window after an invoice due time.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod logic;
mod types;

pub use crate::logic::{is_within_grace, remaining_grace};
pub use crate::types::GraceWindow;
