// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{Client, Result};
use bee_message::{Message, MessageId};
use bee_rest_api::types::responses::MessageMetadataResponse as MessageMetadata;

/// Builder of GET /api/v2/messages/{messageId} endpoint
pub struct GetMessageBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetMessageBuilder<'a> {
    /// Create GET /api/v2/messages endpoint builder
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// GET /api/v2/messages/{messageID} endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message object.
    pub async fn data(self, message_id: &MessageId) -> Result<Message> {
        crate::node_api::core_api::routes::data(self.client, message_id).await
    }

    /// GET /api/v2/messages/{messageID}/metadata endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message metadata.
    pub async fn metadata(self, message_id: &MessageId) -> Result<MessageMetadata> {
        crate::node_api::core_api::routes::metadata(self.client, message_id).await
    }

    /// GET /api/v2/messages/{messageID}/raw endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message raw data.
    pub async fn raw(self, message_id: &MessageId) -> Result<String> {
        crate::node_api::core_api::routes::raw(self.client, message_id).await
    }

    /// GET /api/v2/messages/{messageID}/children endpoint
    /// Consume the builder and returns the list of message IDs that reference a message by its identifier.
    pub async fn children(self, message_id: &MessageId) -> Result<Box<[MessageId]>> {
        crate::node_api::core_api::routes::children(self.client, message_id).await
    }
}
