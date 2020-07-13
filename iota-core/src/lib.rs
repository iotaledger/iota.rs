//! IOTA core

#![deny(unused_extern_crates)]
#![warn(
    //missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

pub use bee_transaction as transaction;
pub use bee_crypto as crypto;
pub use bee_signing as signing;
pub use bee_ternary as ternary;
pub use iota_client as client;

pub use client::Client;

// TODO prelude
