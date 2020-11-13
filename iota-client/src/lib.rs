//! Provides access to the Iota Client API

#![deny(unused_extern_crates)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

#[macro_use]
extern crate serde;

pub mod api;
pub mod builder;
pub mod client;
pub mod error;
pub mod node;
pub mod types;

pub use builder::ClientBuilder;
pub use client::{Client, Topic};
pub use error::*;
pub use reqwest::Url;
pub use types::*;
