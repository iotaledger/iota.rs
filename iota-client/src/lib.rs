// Copyright 2021 IOTA Stiftung
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
pub mod seed;

pub use bee_message;
pub use bee_rest_api::{
    self,
    handlers::{balance_ed25519::BalanceForAddressResponse, output::OutputResponse},
    types::{AddressDto, OutputDto},
};
// pub use bee_signing_ext::{self, binary::BIP32Path,};
pub use builder::ClientBuilder;
pub use client::*;
pub use error::*;
#[cfg(feature = "mqtt")]
pub use node::Topic;
pub use reqwest::Url;
pub use seed::*;

#[cfg(feature = "mqtt")]
mod async_runtime {
    use once_cell::sync::OnceCell;
    use tokio::runtime::Runtime;

    use std::sync::Mutex;

    static RUNTIME: OnceCell<Mutex<Runtime>> = OnceCell::new();

    pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
        let runtime = RUNTIME.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
        runtime.lock().unwrap().block_on(cb)
    }

    pub(crate) fn spawn<F>(future: F)
    where
        F: futures::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let runtime = RUNTIME.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
        runtime.lock().unwrap().spawn(future);
    }
}

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
