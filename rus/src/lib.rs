#![warn(
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    missing_debug_implementations
)]

//! Rust-Unit-System (rus) is a units library built around the idea that it should be
//! simple to work with units in code. Working with units using this library should be similar to
//! working with units in a normal mathematical context.
//!
//! Create quantities by multiplying units with numbers.
//! ```
//! use rus::unit_defs as u;
//! let distance = 12. * u::cm;
//! ```
//!
//! Arithmetic on quantities works as you would expect.
//! ```
//! use rus::unit_defs as u;
//! let distance = 12. * u::cm;
//! let time = 2. * u::s;
//! let velocity1 = distance / time;
//! let velocity2 = 0.06 * u::m / u::s;
//! assert_eq!(velocity1, velocity2);
//! ```
//!
//! Converting between units can be done with the `to` method.
//! ```
//! use rus::unit_defs as u;
//! let distance = 12. * u::cm;
//! let time = 2. * u::s;
//! let velocity1 = (distance / time).to(u::m / u::s);
//! let velocity2 = 0.06 * u::m / u::s;
//! assert_eq!(velocity1, velocity2);
//! ```

mod quantity;
pub use quantity::SingleQuantity;
mod unit;
pub use unit::single::unit_defs;
pub use unit::{composite::CompositeUnit, single::SingleUnit};
