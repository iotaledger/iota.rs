#![deny(unused_extern_crates)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Utilities helpers for Iota

mod seed_random_generator;
mod stopwatch;

/// Provides an adder that sums to slices of trits
pub mod trit_adder;

pub use self::seed_random_generator::generate_new_seed;
pub use self::stopwatch::StopWatch;
