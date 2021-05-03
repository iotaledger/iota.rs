// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use backtrace::Backtrace;
use futures::{Future, FutureExt};
use iota_client::Client;
use neon::prelude::*;
use once_cell::sync::{Lazy, OnceCell};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use tokio::runtime::Runtime;

use std::{
    any::Any,
    collections::HashMap,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{Arc, Mutex, RwLock},
};

mod classes;
use classes::*;

type ClientInstanceMap = Arc<RwLock<HashMap<String, Arc<RwLock<Client>>>>>;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    #[error("`{0}`")]
    Anyhow(#[from] anyhow::Error),
    #[error("`{0}`")]
    Client(#[from] iota_client::Error),
    #[error("`{0}`")]
    Hex(#[from] hex::FromHexError),
    #[error("`{0}`")]
    Panic(String),
    #[error("`{0}`")]
    Message(iota_client::bee_message::Error),
    #[error("`{0}`")]
    SerdeJson(serde_json::Error),
}

impl From<iota_client::bee_message::Error> for Error {
    fn from(error: iota_client::bee_message::Error) -> Self {
        Self::Message(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::SerdeJson(error)
    }
}

pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}

/// Gets the client instances map.
fn instances() -> &'static ClientInstanceMap {
    static INSTANCES: Lazy<ClientInstanceMap> = Lazy::new(Default::default);
    &INSTANCES
}

pub(crate) fn get_client(id: &str) -> Arc<RwLock<Client>> {
    let map = instances()
        .read()
        .expect("failed to lock client instances: get_client()");
    map.get(id).expect("client dropped or not initialised").clone()
}

pub(crate) fn store_client(client: Client) -> String {
    let mut map = instances()
        .write()
        .expect("failed to lock client instances: store_client()");
    let id: String = thread_rng().sample_iter(&Alphanumeric).take(10).collect();
    map.insert(id.clone(), Arc::new(RwLock::new(client)));
    id
}

pub(crate) fn remove_client(id: &str) {
    let mut map = instances()
        .write()
        .expect("failed to lock client instances: remove_client()");
    map.remove(id);
}

fn panic_to_response_message(panic: Box<dyn Any>) -> String {
    let msg = if let Some(message) = panic.downcast_ref::<String>() {
        format!("Internal error: {}", message)
    } else if let Some(message) = panic.downcast_ref::<&str>() {
        format!("Internal error: {}", message)
    } else {
        "Internal error".to_string()
    };
    let current_backtrace = Backtrace::new();
    format!("{}\n\n{:?}", msg, current_backtrace)
}

pub(crate) async fn convert_async_panics<T, F: Future<Output = Result<T>>>(f: impl FnOnce() -> F) -> Result<T> {
    match AssertUnwindSafe(f()).catch_unwind().await {
        Ok(result) => result,
        Err(panic) => Err(Error::Panic(panic_to_response_message(panic))),
    }
}

pub(crate) fn convert_panics<T, F: FnOnce() -> Result<T>>(f: F) -> Result<T> {
    match catch_unwind(AssertUnwindSafe(|| f())) {
        Ok(result) => result,
        Err(panic) => Err(Error::Panic(panic_to_response_message(panic))),
    }
}

register_module!(mut cx, {
    cx.export_class::<JsClientBuilder>("ClientBuilder")?;
    cx.export_class::<JsClient>("Client")?;
    cx.export_class::<JsTopicSubscriber>("TopicSubscriber")?;
    cx.export_class::<JsMessageGetter>("MessageGetter")?;
    cx.export_class::<JsMessageSender>("MessageSender")?;
    cx.export_class::<JsUnspentAddressGetter>("UnspentAddressGetter")?;
    cx.export_class::<JsAddressGetter>("AddressGetter")?;
    cx.export_class::<JsBalanceGetter>("BalanceGetter")?;
    Ok(())
});
