// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::message_interface::{message_type::MessageType, response_type::ResponseType};

/// The actor response type.
#[derive(Serialize, Debug)]
pub struct Response {
    #[serde(flatten)]
    response: ResponseType,
    action: MessageType,
}

impl Response {
    /// Creates a new response.
    pub fn new(action: MessageType, response: ResponseType) -> Self {
        Self { response, action }
    }

    /// The response's type.
    pub fn response(&self) -> &ResponseType {
        &self.response
    }
}
