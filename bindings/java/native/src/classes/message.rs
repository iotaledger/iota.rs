// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::{cell::RefCell, rc::Rc};


use getset::{CopyGetters, Getters};
use iota_client::bee_message::{
    prelude::{Message as RustMessage, MessageBuilder as RustMessageBuilder, MessageId, Parents}
};

use anyhow::anyhow;

use crate::{
    Result, Payload
};


#[derive(Clone, PartialEq)]
pub struct MessageWrap {
    message: Message,

    message_id: MessageId,
}

impl MessageWrap {
    pub fn new(message_id: MessageId, message: Message) -> Self {
        Self {
            message,
            message_id
        }
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
        write!(f, "message_id={}, message={}", 
            self.message_id, self.message)
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
        write!(f, "network_id={}, nonce={}, id={}, payload={:?}, parents=({:?})", 
            self.network_id, self.nonce, self.id, self.payload, self.parents)
    }
}

impl core::fmt::Debug for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Message({})", self)
    }
}
/*
impl Clone for Message {
    fn clone(&self) -> Self {
        Message {
            network_id: self.network_id,
            parents: self.parents().clone(),
            payload: self.payload.clone(),
            nonce: self.nonce,
            id: self.id.clone(),
        }
    }
}*/

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
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_network_id(network_id);
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
    /*pub fn nonce_provider(&self, nonce_provider: P, target_score: f64) -> Self {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_payload(payload.to_inner())
            .unwrap();
        MessageBuilder::new_with_builder(new_builder)
    }*/

    /// Finishes the `MessageBuilder` into a `Message`.
    pub fn finish(&self) -> Result<Message> {
        let msg = self.builder.borrow_mut().take().unwrap().finish();
        match msg {
            Ok(m) => Ok(m.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}
