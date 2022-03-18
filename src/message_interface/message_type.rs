// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::message_interface::ClientMethod;
use serde::{ser::Serializer, Deserialize, Serialize};

/// The messages that can be sent to the message interface.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "cmd", content = "payload")]
pub enum MessageType {
    /// Consume a client method.
    CallClientMethod(ClientMethod),
}

impl Serialize for MessageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            MessageType::CallClientMethod { .. } => {
                serializer.serialize_unit_variant("MessageType", 0, "CallClientMethod")
            }
        }
    }
}
