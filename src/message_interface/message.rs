// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{ser::Serializer, Deserialize, Serialize};

use crate::message_interface::ClientMethod;

/// The messages that can be sent to the message interface.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "cmd", content = "payload")]
pub enum Message {
    /// Consume a client method.
    CallClientMethod(ClientMethod),
}

impl Serialize for Message {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Message::CallClientMethod { .. } => serializer.serialize_unit_variant("Message", 0, "CallClientMethod"),
        }
    }
}
