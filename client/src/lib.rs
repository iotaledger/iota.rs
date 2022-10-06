// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A general purpose IOTA client library for interaction with the IOTA network (Tangle)
//!
//! High-level functions are accessible via the [`Client`][client::Client].
//!
//! ## Sending a block without a payload
//!  ```no_run
//! # use iota_client::{Client, Result};
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let client = Client::builder()
//!    .with_node("http://localhost:14265")?
//!    .finish()?;
//!
//! let block = client
//!    .block()
//!    .finish()
//!    .await?;
//!
//! println!("Block sent {}", block.id());
//! # Ok(())}
//! ```

#![deny(unused_extern_crates)]
#![warn(missing_docs, rust_2018_idioms, unreachable_pub)]

#[macro_use]
extern crate serde;

#[cfg(feature = "mqtt")]
macro_rules! lazy_static {
    ($init:expr => $type:ty) => {{
        static mut VALUE: Option<$type> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        INIT.call_once(|| unsafe { VALUE = Some($init) });
        unsafe { VALUE.as_ref() }.expect("failed to get lazy static value")
    }};
}

pub mod api;
pub mod client;
pub mod constants;
pub mod db;
pub mod error;
#[cfg(feature = "message_interface")]
pub mod message_interface;
pub mod node_api;
pub mod node_manager;
pub mod secret;
#[cfg(feature = "stronghold")]
pub mod stronghold;
pub mod utils;

pub use bee_api_types as api_types;
pub use bee_block as block;
pub use bee_pow as pow;
pub use crypto::{self, keys::slip10::Seed};
pub use packable;
pub use url::Url;

#[cfg(feature = "mqtt")]
pub use self::node_api::mqtt::*;
pub use self::{client::*, error::*, node_api::core::routes::NodeInfoWrapper, utils::*};

#[cfg(feature = "mqtt")]
mod async_runtime {
    use std::sync::Mutex;

    use once_cell::sync::OnceCell;
    use tokio::runtime::Runtime;

    static RUNTIME: OnceCell<Mutex<Runtime>> = OnceCell::new();

    pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
        let runtime = RUNTIME.get_or_init(|| Mutex::new(Runtime::new().expect("failed to create Tokio runtime")));
        runtime.lock().expect("failed to lock the runtime.").block_on(cb)
    }

    pub(crate) fn spawn<F>(future: F)
    where
        F: futures::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let runtime = RUNTIME.get_or_init(|| Mutex::new(Runtime::new().expect("failed to create Tokio runtime")));
        runtime.lock().expect("failed to lock the runtime.").spawn(future);
    }
}
