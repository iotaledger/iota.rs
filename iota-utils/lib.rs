#![deny(unused_extern_crates)]

mod seed_random_generator;
mod stopwatch;

/// Provides an adder that sums to slices of trits
pub mod trit_adder;

pub use self::seed_random_generator::generate_new_seed;
pub use self::stopwatch::StopWatch;
