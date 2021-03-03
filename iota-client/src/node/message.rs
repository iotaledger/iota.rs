// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{log_request, parse_response, Client, Error, Result};
use bee_message::{Message, MessageId};
use bee_rest_api::{
    endpoints::api::v1::{
        message_children::MessageChildrenResponse, message_metadata::MessageMetadataResponse as MessageMetadata,
        messages_find::MessagesForIndexResponse,
    },
    types::MessageDto,
};

use log::info;

use std::convert::TryFrom;

/// Builder of GET /api/v1/messages/{messageId} endpoint
pub struct GetMessageBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetMessageBuilder<'a> {
    /// Create GET /api/v1/messages endpoint builder
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// GET /api/v1/messages?index={Index} endpoint
    /// Consume the builder and search for messages matching the index
    pub async fn index<I: AsRef<[u8]>>(self, index: I) -> Result<Box<[MessageId]>> {
        let mut url = self.client.get_node()?;
        let path = "api/v1/messages";
        url.set_path(path);
        url.set_query(Some(&format!("index={}", hex::encode(index))));
        let resp = reqwest::get(url).await?;

        #[derive(Debug, Serialize, Deserialize)]
        struct MessagesWrapper {
            data: MessagesForIndexResponse,
        }
        log_request!("GET", path, resp);
        parse_response!(resp, 200 => {
            let ids = resp.json::<MessagesWrapper>().await?;
            ids.data.message_ids
                .iter()
                .map(|s| {
                    let mut message_id = [0u8; 32];
                    hex::decode_to_slice(s, &mut message_id)?;
                    Ok(MessageId::from(message_id))
                })
                .collect::<Result<Box<[MessageId]>>>()
        })
    }

    /// GET /api/v1/messages/{messageID} endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message object.
    pub async fn data(self, message_id: &MessageId) -> Result<Message> {
        let mut url = self.client.get_node()?;
        let path = &format!("api/v1/messages/{}", message_id);
        url.set_path(path);
        let resp = reqwest::get(url).await?;

        #[derive(Debug, Serialize, Deserialize)]
        struct MessagesWrapper {
            data: MessageDto,
        }
        log_request!("GET", path, resp);
        parse_response!(resp, 200 => {
            let meta = resp.json::<MessagesWrapper>().await?;
            Ok(
                Message::try_from(&meta.data).expect("Can't convert MessageDto to Message"))
        })
    }

    /// GET /api/v1/messages/{messageID}/metadata endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message metadata.
    pub async fn metadata(self, message_id: &MessageId) -> Result<MessageMetadata> {
        let mut url = self.client.get_node()?;
        let path = &format!("api/v1/messages/{}/metadata", message_id);
        url.set_path(path);
        let resp = reqwest::get(url).await?;
        #[derive(Debug, Serialize, Deserialize)]
        struct MessagesWrapper {
            data: MessageMetadata,
        }
        log_request!("GET", path, resp);
        parse_response!(resp, 200 => {
            let meta = resp.json::<MessagesWrapper>().await?;
            Ok(meta.data)
        })
    }

    /// GET /api/v1/messages/{messageID}/children endpoint
    /// Consume the builder and find a message by its identifer. This method returns the given message raw data.
    pub async fn raw(self, message_id: &MessageId) -> Result<String> {
        let mut url = self.client.get_node()?;
        let path = &format!("api/v1/messages/{}/raw", message_id);
        url.set_path(path);
        let resp = reqwest::get(url).await?;

        log_request!("GET", path, resp);
        parse_response!(resp, 200 => {
            Ok(resp.text().await?)
        })
    }

    /// Consume the builder and returns the list of message IDs that reference a message by its identifier.
    pub async fn children(self, message_id: &MessageId) -> Result<Box<[MessageId]>> {
        let mut url = self.client.get_node()?;
        let path = &format!("api/v1/messages/{}/children", message_id);
        url.set_path(path);
        let resp = reqwest::get(url).await?;

        #[derive(Debug, Serialize, Deserialize)]
        struct MessagesWrapper {
            data: MessageChildrenResponse,
        }
        log_request!("GET", path, resp);
        crate::parse_response!(resp, 200 => {
            let meta = resp.json::<MessagesWrapper>().await?;
            meta.data.children_message_ids
                .iter()
                .map(|s| {
                    let mut message_id = [0u8; 32];
                    hex::decode_to_slice(s, &mut message_id)?;
                    Ok(MessageId::from(message_id))
                })
                .collect::<Result<Box<[MessageId]>>>()
        })
    }
}
