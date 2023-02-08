// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Message interface for bindings

mod message;
mod message_handler;
mod response;

pub use self::{message::Message, message_handler::ClientMessageHandler, response::Response};
use crate::{ClientBuilder, Result};

/// Create message handler with client options
pub fn create_message_handler(client_config: Option<String>) -> Result<ClientMessageHandler> {
    let client = match client_config {
        Some(options) => ClientBuilder::new().from_json(&options)?.finish()?,
        None => ClientBuilder::new().finish()?,
    };
    Ok(ClientMessageHandler::with_client(client))
}
