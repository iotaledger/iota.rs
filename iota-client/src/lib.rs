// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

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

pub use bee_signing_ext::{binary::BIP32Path, Seed};
pub use builder::ClientBuilder;
pub use client::{BrokerOptions, Client, Topic, TopicEvent};
pub use error::*;
pub use reqwest::Url;
pub use types::*;
