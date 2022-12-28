#![warn(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    missing_debug_implementations
)]

mod quantity;
mod unit;
pub use unit::single::unit_defs::*;
