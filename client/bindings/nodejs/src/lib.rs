// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::needless_borrow)]

mod message_handler;
use fern_logger::{logger_init, LoggerConfig, LoggerOutputConfigBuilder};
pub use message_handler::*;
use neon::prelude::*;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;
pub static RUNTIME: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

pub fn init_logger(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let config = cx.argument::<JsString>(0)?.value(&mut cx);
    let output_config: LoggerOutputConfigBuilder = serde_json::from_str(&config).expect("invalid logger config");
    let config = LoggerConfig::build().with_output(output_config).finish();
    logger_init(config).expect("failed to init logger");
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    // Message handler methods.
    cx.export_function("sendMessage", message_handler::send_message)?;
    cx.export_function("messageHandlerNew", message_handler::message_handler_new)?;

    // MQTT
    cx.export_function("listen", message_handler::listen)?;

    cx.export_function("initLogger", init_logger)?;
    Ok(())
}
