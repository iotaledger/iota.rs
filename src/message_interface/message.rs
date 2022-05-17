// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use tokio::sync::mpsc::UnboundedSender;

use super::{message_type::MessageType, response::Response};

/// The message type.
#[derive(Debug, Clone)]
pub struct Message {
    pub(crate) message_type: MessageType,
    pub(crate) response_tx: UnboundedSender<Response>,
}

impl Message {
    /// Create a new instance of Message.
    pub fn new(message_type: MessageType, response_tx: UnboundedSender<Response>) -> Self {
        Self {
            message_type,
            response_tx,
        }
    }

    /// Get the message type.
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }
}
