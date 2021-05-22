// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::{cell::RefCell, rc::Rc};

use getset::{CopyGetters, Getters};
use iota_client::{
    api::ClientMessageBuilder as RustClientMessageBuilder,
    bee_message::prelude::{Message as RustMessage, MessageBuilder as RustMessageBuilder, MessageId, Parents},
    node::GetMessageBuilder as RustGetMessageBuilder,
    Seed as RustSeed,
};

use anyhow::anyhow;

use crate::{
    bee_types::{MessageMetadata, UtxoInput},
    full_node_api::Client,
    Payload, Result,
};

#[derive(Clone, PartialEq)]
pub struct MessageWrap {
    message: Message,

    message_id: MessageId,
}

impl MessageWrap {
    pub fn new(message_id: MessageId, message: Message) -> Self {
        Self { message, message_id }
    }

    pub fn message(&self) -> Message {
        self.message.clone()
    }

    pub fn message_id(&self) -> MessageId {
        self.message_id.clone()
    }
}

impl core::fmt::Display for MessageWrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "message_id={}, message={}", self.message_id, self.message)
    }
}

impl core::fmt::Debug for MessageWrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "MessageWrap({})", self)
    }
}

#[derive(Clone, PartialEq, Getters, CopyGetters)]
pub struct Message {
    /// Specifies which network this message is meant for.
    #[getset(get_copy = "pub")]
    network_id: u64,
    /// The [`MessageId`]s that this message directly approves.
    parents: Vec<MessageId>,
    /// The optional [Payload] of the message.
    payload: Option<Payload>,
    /// The result of the Proof of Work in order fot the message to be accepted into the tangle.
    #[getset(get_copy = "pub")]
    nonce: u64,

    id: MessageId,
}

impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "network_id={}, nonce={}, id={}, payload={:?}, parents=({:?})",
            self.network_id, self.nonce, self.id, self.payload, self.parents
        )
    }
}

impl core::fmt::Debug for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Message({})", self)
    }
}
// impl Clone for Message {
// fn clone(&self) -> Self {
// Message {
// network_id: self.network_id,
// parents: self.parents().clone(),
// payload: self.payload.clone(),
// nonce: self.nonce,
// id: self.id.clone(),
// }
// }
// }

impl From<RustMessage> for Message {
    fn from(message: RustMessage) -> Self {
        let payload: Option<Payload> = match message.payload() {
            Some(p) => Some(p.clone().into()),
            None => None,
        };
        Self {
            network_id: message.network_id(),
            parents: message.parents().to_vec(),
            payload: payload,
            nonce: message.nonce(),
            id: message.id().0,
        }
    }
}

impl Message {
    pub fn builder() -> MessageBuilder {
        MessageBuilder::new()
    }

    pub fn id(&self) -> MessageId {
        self.id.clone()
    }

    pub fn parents(&self) -> Vec<MessageId> {
        self.parents.clone()
    }

    pub fn payload(&self) -> Option<Payload> {
        self.payload.clone()
    }
}

pub struct MessageBuilder {
    builder: Rc<RefCell<Option<RustMessageBuilder>>>,
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(RustMessageBuilder::default()))),
        }
    }
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder::new_with_builder(RustMessageBuilder::new())
    }

    fn new_with_builder(builder: RustMessageBuilder) -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(builder))),
        }
    }

    /// Adds a network id to a `MessageBuilder`.
    pub fn network_id(&self, network_id: u64) -> Self {
        let new_builder = self.builder.borrow_mut().take().unwrap().with_network_id(network_id);
        MessageBuilder::new_with_builder(new_builder)
    }

    /// Adds parents to a `MessageBuilder`.
    pub fn parents(&self, parents: Vec<MessageId>) -> Result<Self> {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_parents(Parents::new(parents)?);
        Ok(MessageBuilder::new_with_builder(new_builder))
    }

    /// Adds a payload to a `MessageBuilder`.
    pub fn payload(&self, payload: Payload) -> Self {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_payload(payload.to_inner());
        MessageBuilder::new_with_builder(new_builder)
    }

    /// Adds a nonce provider to a `MessageBuilder`.
    // pub fn nonce_provider(&self, nonce_provider: P, target_score: f64) -> Self {
    // let new_builder = self
    // .builder
    // .borrow_mut()
    // .take()
    // .unwrap()
    // .with_payload(payload.to_inner())
    // .unwrap();
    // MessageBuilder::new_with_builder(new_builder)
    // }

    /// Finishes the `MessageBuilder` into a `Message`.
    pub fn finish(&self) -> Result<Message> {
        let msg = self.builder.borrow_mut().take().unwrap().finish();
        match msg {
            Ok(m) => Ok(m.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}

pub struct ClientMessageBuilderInternal<'a> {
    seed: Option<RustSeed>,
    builder: RustClientMessageBuilder<'a>,
}

pub struct ClientMessageBuilder<'a> {
    fields: Rc<RefCell<Option<ClientMessageBuilderInternal<'a>>>>,
}

impl<'a> ClientMessageBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        let internal = ClientMessageBuilderInternal {
            seed: None,
            builder: RustClientMessageBuilder::new(client.borrow()),
        };
        Self {
            fields: Rc::new(RefCell::new(Option::from(internal))),
        }
    }

    fn new_with_fields(fields: ClientMessageBuilderInternal<'a>) -> Self {
        Self {
            fields: Rc::new(RefCell::new(Option::from(fields))),
        }
    }

    /// Sets the seed.
    pub fn with_seed(&self, seed: &str) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.seed = Some(RustSeed::from_bytes(seed.as_bytes()));
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Sets the account index.
    pub fn with_account_index(&self, account_index: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.builder = fields.builder.with_account_index(account_index);
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Sets the index of the address to start looking for balance.
    pub fn with_initial_address_index(&self, initial_address_index: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.builder = fields.builder.with_initial_address_index(initial_address_index);
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Set a custom input(transaction output)
    pub fn with_input(&self, input: UtxoInput) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.builder = fields.builder.with_input(input.to_inner_clone());
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Set a custom range in which to search for addresses for custom inputs. Default: 0..100
    pub fn with_input_range(&self, low: usize, high: usize) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.builder = fields.builder.with_input_range(low..high);
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Set a transfer to the builder
    pub fn with_output(&self, address: &str, amount: u64) -> Result<Self> {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        let ret = fields.builder.with_output(address, amount);

        match ret {
            Ok(b) => {
                fields.builder = b;
                Ok(ClientMessageBuilder::new_with_fields(fields))
            }
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Set a dust allowance transfer to the builder, address needs to be Bech32 encoded
    pub fn with_dust_allowance_output(&self, address: &str, amount: u64) -> Result<Self> {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        let ret = fields.builder.with_dust_allowance_output(address, amount);

        match ret {
            Ok(b) => {
                fields.builder = b;
                Ok(ClientMessageBuilder::new_with_fields(fields))
            }
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Set a transfer to the builder, address needs to be hex encoded
    pub fn with_output_hex(&self, address: &str, amount: u64) -> Result<Self> {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        let ret = fields.builder.with_output_hex(address, amount);

        match ret {
            Ok(b) => {
                fields.builder = b;
                Ok(ClientMessageBuilder::new_with_fields(fields))
            }
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    /// Set indexation to the builder
    pub fn with_index_vec(&self, index: Vec<u8>) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.builder = fields.builder.with_index(index.clone());
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Set indexation to the builder
    pub fn with_index_string(&self, index: &str) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.builder = fields.builder.with_index(index.to_string().as_bytes());
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Set data to the builder
    pub fn with_data(&self, data: Vec<u8>) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.builder = fields.builder.with_data(data.clone());
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Set data to the builder
    pub fn with_data_string(&self, data: &str) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.builder = fields.builder.with_data(data.to_string().as_bytes().to_vec());
        ClientMessageBuilder::new_with_fields(fields)
    }

    pub fn finish(&self) -> Result<Message> {
        let inner = self.fields.borrow_mut().take().unwrap();
        let res = crate::block_on(async {
            if let Some(s) = inner.seed {
                inner.builder.with_seed(&s).finish().await
            } else {
                inner.builder.finish().await
            }
        });
        match res {
            Ok(m) => Ok(m.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}

pub struct GetMessageBuilder<'a> {
    client: &'a Client,
}

impl<'a> GetMessageBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client: client }
    }

    pub fn index_string(&self, index: &str) -> Result<Vec<MessageId>> {
        let res = crate::block_on(async {
            RustGetMessageBuilder::new(self.client.borrow())
                .index(index.to_string().as_bytes())
                .await
        });
        match res {
            Ok(r) => Ok(r.iter().map(|message_id| message_id.clone().into()).collect()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn index_vec(&self, index: Vec<u8>) -> Result<Vec<MessageId>> {
        let res = crate::block_on(async {
            RustGetMessageBuilder::new(self.client.borrow())
                .index(index.clone())
                .await
        });
        match res {
            Ok(r) => Ok(r.iter().map(|message_id| message_id.clone().into()).collect()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn data(&self, message_id: MessageId) -> Result<Message> {
        let res = crate::block_on(async { RustGetMessageBuilder::new(self.client.borrow()).data(&message_id).await });
        match res {
            Ok(r) => Ok(r.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn metadata(&self, message_id: MessageId) -> Result<MessageMetadata> {
        let res = crate::block_on(async {
            RustGetMessageBuilder::new(self.client.borrow())
                .metadata(&message_id)
                .await
        });
        match res {
            Ok(r) => Ok(r.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn raw(&self, message_id: MessageId) -> Result<String> {
        let res = crate::block_on(async { RustGetMessageBuilder::new(self.client.borrow()).raw(&message_id).await });
        match res {
            Ok(r) => Ok(r),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn children(&self, message_id: MessageId) -> Result<Vec<MessageId>> {
        let res = crate::block_on(async {
            RustGetMessageBuilder::new(self.client.borrow())
                .children(&message_id)
                .await
        });
        match res {
            Ok(r) => Ok(r.iter().map(|message_id| message_id.clone().into()).collect()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}
