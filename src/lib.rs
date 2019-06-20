#![deny(unused_extern_crates)]

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

mod account;
pub mod iota_api;
pub mod prelude;

pub use iota_client;
pub use iota_constants;
pub use iota_conversion;
pub use iota_crypto;
pub use iota_model;
pub use iota_pow;
pub use iota_signing;
pub use iota_utils;
pub use iota_validation;

type Result<T> = ::std::result::Result<T, failure::Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
