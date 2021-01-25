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

pub use bee_signing_ext::{binary::BIP32Path, Seed};
pub use builder::ClientBuilder;
pub use client::*;
pub use error::*;
#[cfg(feature = "mqtt")]
pub use reqwest::Url;

/// match a response with an expected status code or return the default error variant.
#[macro_export]
macro_rules! parse_response {
    ($response:ident, $expected_status:pat => $ok:block) => {{
        match $response.status().as_u16() {
            $expected_status => $ok,
            status => Err(Error::ResponseError(status, $response.text().await?)),
        }
    }};
}

/// gets the BIP32 account path from a given account_index/address_internal/address_index
#[macro_export]
macro_rules! account_path {
    ($account_index:expr) => {
        format!("m/44'/4218'/{}'", $account_index)
    };
}
