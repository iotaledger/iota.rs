//! Provides access to the Iota Client API

#![deny(unused_extern_crates)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

#[macro_use]
extern crate serde;

pub mod api;
pub mod builder;
#[macro_use]
pub mod client;
pub mod error;
pub mod node;
pub mod types;

pub use builder::ClientBuilder;
pub use client::Client;
pub use error::*;
pub use reqwest::Url;
pub use types::*;
