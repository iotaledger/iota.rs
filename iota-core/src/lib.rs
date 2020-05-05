//! IOTA core

#![deny(unused_extern_crates)]
#![warn(
    //missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

pub use iota_bundle_preview as bundle;
pub use iota_client as client;
pub use iota_crypto_preview as crypto;
pub use iota_signing_preview as signing;
pub use iota_ternary_preview as ternary;

pub use client::Client;

// TODO prelude
