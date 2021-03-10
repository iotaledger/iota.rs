//! Provides access to the Iota Client API

#![deny(unused_extern_crates)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod builder;
#[macro_use]
pub mod client;
pub mod chrysalis2;
pub mod core;
pub mod error;
pub mod extended;
pub mod migration;
// #[cfg(feature = "quorum")]
// pub mod quorum;
pub mod response;
mod util;

pub use crate::core as iota_core;
pub use builder::ClientBuilder;
pub use chrysalis2::*;
pub use client::Client;
pub use error::*;
pub use extended::GetAccountDataForMigrationBuilder;
pub use migration::*;
pub use response::*;
pub use util::{bytes_to_trytes, str_to_trytes};
