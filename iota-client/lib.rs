//! Provides access to the Iota Client API

#![deny(unused_extern_crates)]
#![warn(
    //missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

#[macro_use]
pub mod client;
pub mod core;
pub mod response;
mod util;

pub use client::Client;
