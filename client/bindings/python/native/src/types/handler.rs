// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::message_interface::ClientMessageHandler as RustClientMessageHandler;
use pyo3::prelude::*;

#[pyclass]
/// The Client Message Handler for message sending.
pub struct ClientMessageHandler {
    /// The client message handler.
    pub client_message_handler: RustClientMessageHandler,
}
