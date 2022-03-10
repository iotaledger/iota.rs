// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{signing::SignerHandle, Client, Result};

use backtrace::Backtrace;
use futures::{Future, FutureExt};

use std::{
    any::Any,
    panic::{catch_unwind, AssertUnwindSafe},
};

use crate::message_interface::{
    client_method::ClientMethod, message::Message, message_type::MessageType, response::Response,
    response_type::ResponseType,
};

fn panic_to_response_message(panic: Box<dyn Any>) -> ResponseType {
    let msg = if let Some(message) = panic.downcast_ref::<String>() {
        format!("Internal error: {}", message)
    } else if let Some(message) = panic.downcast_ref::<&str>() {
        format!("Internal error: {}", message)
    } else {
        "Internal error".to_string()
    };
    let current_backtrace = Backtrace::new();
    ResponseType::Panic(format!("{}\n\n{:?}", msg, current_backtrace))
}

fn convert_panics<F: FnOnce() -> Result<ResponseType>>(f: F) -> Result<ResponseType> {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(result) => result,
        Err(panic) => Ok(panic_to_response_message(panic)),
    }
}

async fn convert_async_panics<F>(f: impl FnOnce() -> F) -> Result<ResponseType>
where
    F: Future<Output = Result<ResponseType>>,
{
    match AssertUnwindSafe(f()).catch_unwind().await {
        Ok(result) => result,
        Err(panic) => Ok(panic_to_response_message(panic)),
    }
}

/// The Client message handler.
pub struct ClientMessageHandler {
    client: Client,
}

impl ClientMessageHandler {
    /// Creates a new instance of the message handler with the default client manager.
    pub async fn new() -> Result<Self> {
        let instance = Self {
            client: Client::builder().finish().await?,
        };
        Ok(instance)
    }

    /// Creates a new instance of the message handler with the specified client.
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// Handle messages
    pub async fn handle(&self, mut message: Message) {
        let response: Result<ResponseType> = match message.message_type_mut() {
            MessageType::CallClientMethod { method } => {
                convert_async_panics(|| async { self.call_client_method(method).await }).await
            }
        };

        let response = match response {
            Ok(r) => r,
            Err(e) => ResponseType::Error(e),
        };
        let _ = message.response_tx.send(Response::new(message.message_type, response));
    }

    async fn call_client_method(&self, method: &ClientMethod) -> Result<ResponseType> {
        match method {
            ClientMethod::GenerateAddresses { signer, options } => {
                let signer = SignerHandle::from_str(signer)?;
                let mut address_builder = self.client.get_addresses(&signer);

                if let Some(options) = options {
                    if let Some(coin_type) = options.coin_type {
                        address_builder = address_builder.with_coin_type(coin_type);
                    };

                    if let Some(account_index) = options.account_index {
                        address_builder = address_builder.with_account_index(account_index);
                    }

                    if let Some(range) = &options.range {
                        address_builder = address_builder.with_range(range.clone());
                    };

                    if let Some(bech32_hrp) = &options.bech32_hrp {
                        address_builder = address_builder.with_bech32_hrp(bech32_hrp.clone());
                    };
                }

                let addresses = address_builder.finish().await?;
                Ok(ResponseType::GeneratedAddresses(addresses))
            }
        }
    }
}
