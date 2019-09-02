#![deny(unused_extern_crates)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Validation used throughout iota related crates

// #[macro_use]
// extern crate failure;
// #[macro_use]
// extern crate lazy_static;
// #[macro_use]
// extern crate serde;
// #[macro_use]
// extern crate serde_json;

//mod account;

/// Provides useful imports
pub mod prelude;
/// Provides access to the Iota Client API
pub use iota_client;
/// Constants used throughout these crates
pub use iota_constants;
/// Conversion Trait and some helper methods
pub use iota_conversion;
/// Crypto used by Iota
pub use iota_crypto;
/// Structs representing the various data types used by Iota
pub use iota_model;
/// Proof of Work
pub use iota_pow;
/// Methods facilitating signing for Iota
pub use iota_signing;
/// Various utils
pub use iota_utils;
/// Validation used throughout the crates
pub use iota_validation;
