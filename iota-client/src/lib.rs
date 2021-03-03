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
#[cfg(feature = "storage")]
#[cfg_attr(docsrs, doc(cfg(feature = "storage")))]
pub mod storage;

pub use bee_message;
pub use bee_rest_api::{
    self,
    endpoints::api::v1::{balance_ed25519::BalanceForAddressResponse, output::OutputResponse},
    types::{AddressDto, OutputDto},
};
// pub use bee_signing_ext::{self, binary::BIP32Path,};
pub use builder::ClientBuilder;
pub use client::*;
pub use crypto::slip10::Seed;
pub use error::*;
#[cfg(feature = "mqtt")]
pub use node::Topic;
pub use node::{OutputType, OutputsOptions as AddressOutputsOptions};
#[cfg(feature = "storage")]
pub use storage::*;
pub use url::Url;

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

/// Log info about the request and response.
#[macro_export]
macro_rules! log_request {
    ($method: expr, $url: expr, $response:ident) => {
        info!(
            "Request method: {}
            Request URL: {}",
            // Response status: {}",
            $method,
            $url,
            // $response.status().as_u16()
        );
    };
}
