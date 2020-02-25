//! Provides access to the Iota Client API

#![deny(unused_extern_crates)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

#[macro_use]
extern crate serde;

pub mod client;
pub mod request;
pub mod response;

pub use client::Client;
