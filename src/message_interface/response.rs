// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

use crate::message_interface::{message_type::MessageType, response_type::ResponseType};

/// The actor response type.
#[derive(Serialize, Debug)]
pub struct Response<'a> {
    #[serde(flatten)]
    response_type: ResponseType<'a>,
    action: MessageType,
}

impl<'a> Response<'a> {
    /// Creates a new response.
    pub fn new(action: MessageType, response_type: ResponseType<'a>) -> Self {
        Self { response_type, action }
    }

    /// The response's type.
    pub fn response_type(&self) -> &ResponseType<'a> {
        &self.response_type
    }
}
