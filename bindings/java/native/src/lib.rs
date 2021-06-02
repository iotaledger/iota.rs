// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// Used in verifying correct binding
mod jni_c_header;
pub mod verifylink;

mod bee_types;
mod classes;
mod java_glue;

pub use crate::{bee_types::*, classes::*, java_glue::*};

pub use anyhow::{Error, Result};

use once_cell::sync::OnceCell;
use std::sync::Mutex;
use tokio::runtime::Runtime;

pub(crate) fn block_on<C: futures::Future>(cb: C) -> C::Output {
    static INSTANCE: OnceCell<Mutex<Runtime>> = OnceCell::new();
    let runtime = INSTANCE.get_or_init(|| Mutex::new(Runtime::new().unwrap()));
    runtime.lock().unwrap().block_on(cb)
}
