// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A general purpose IOTA client library for interaction with the IOTA network (Tangle)
//!
//! High-level functions are accessible via the [`Client`][client::Client].
//!
//! ## Sending a message with an indexation payload
//!  ```compile_fail
//! let iota = Client::builder()
//!    .with_node("http://localhost:14265")?
//!    .finish()
//!    .await?;
//!
//! let message = iota
//!    .message()
//!    .with_index("Hello")
//!    .with_data("Tangle".as_bytes().to_vec())
//!    .finish()
//!    .await?;
//!
//! println!("Message sent {}", message.id());
//! ```

#![deny(unused_extern_crates)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

#[macro_use]
extern crate serde;

macro_rules! lazy_static {
    ($init:expr => $type:ty) => {{
        static mut VALUE: Option<$type> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        INIT.call_once(|| unsafe { VALUE = Some($init) });
        unsafe { VALUE.as_ref() }.expect("failed to get lazy static value")
    }};
}

pub mod api;
pub mod builder;
pub mod client;
pub mod error;
// todo: rename and only have high level functions or remove completely
pub mod node;
pub mod node_api;
pub mod node_manager;
pub mod signing;
pub mod utils;

pub use bee_common as common;
pub use bee_message;
pub use bee_pow as pow;
pub use bee_rest_api;
pub use builder::ClientBuilder;
pub use client::*;
pub use crypto::{self, keys::slip10::Seed};
pub use error::*;
pub use node::{OutputType, OutputsOptions as AddressOutputsOptions};
#[cfg(feature = "mqtt")]
pub use node_api::mqtt::*;
pub use packable;
pub use url::Url;
pub use utils::*;

#[cfg(feature = "mqtt")]
mod async_runtime {
    use once_cell::sync::OnceCell;
    use tokio::runtime::Runtime;

    use std::sync::Mutex;

    static RUNTIME: OnceCell<Mutex<Runtime>> = OnceCell::new();

    pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
        let runtime = RUNTIME.get_or_init(|| Mutex::new(Runtime::new().expect("Failed to create Tokio runtim")));
        runtime.lock().expect("Failed to lock the runtime.").block_on(cb)
    }

    pub(crate) fn spawn<F>(future: F)
    where
        F: futures::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let runtime = RUNTIME.get_or_init(|| Mutex::new(Runtime::new().expect("Failed to create Tokio runtim")));
        runtime.lock().expect("Failed to lock the runtime.").spawn(future);
    }
}
