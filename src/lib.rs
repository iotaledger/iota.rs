#![deny(unused_extern_crates)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

// mod account;

/// Helpers for using the Iota Client API
pub mod iota_api;
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

/// Error type used throughout the crates
type Result<T> = ::std::result::Result<T, failure::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
