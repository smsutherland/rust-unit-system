#![warn(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    missing_debug_implementations
)]

//! units (name pending) is a units library built around the idea that it should be
//! simple to work with units in code. Working with units using this library should be similar to
//! working with units in a normal mathematical context.

mod quantity;
mod unit;
pub use unit::single::unit_defs::*;
