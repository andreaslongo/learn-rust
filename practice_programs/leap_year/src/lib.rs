//! The leap year coding challenge

// Lints:
#![warn(clippy::pedantic)]
#![warn(deprecated_in_future)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

mod as_function;
mod as_struct;

pub use as_function::is_leap_year;
pub use as_struct::Year;
