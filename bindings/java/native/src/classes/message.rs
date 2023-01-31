// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use anyhow::anyhow;
use getset::{CopyGetters, Getters};
use iota_client::{
    api::{ClientMessageBuilder as RustClientMessageBuilder, PreparedTransactionData as RustPreparedTransactionData},
    bee_message::{
        payload::Payload,
        prelude::{Message as RustMessage, MessageBuilder as RustMessageBuilder, MessageId, Parents},
    },
    node::GetMessageBuilder as RustGetMessageBuilder,
    ClientMiner as RustClientMiner, Seed as RustSeed,
};

use crate::{
    bee_types::{
        IndexationPayload, MessageMetadata, MilestonePayload, ReceiptPayload, TransactionPayload, TreasuryPayload,
        UtxoInput,
    },
    full_node_api::Client,
    prepared::{addres_into_rust_address_recorder, PreparedTransactionData},
    ClientMiner, MessagePayload, Result,
};

#[derive(Clone, Eq, PartialEq)]
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
        self.message_id
    }
}

impl core::fmt::Display for MessageWrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "message_id={}, message={}", self.message_id, self.message)
    }
}

impl core::fmt::Debug for MessageWrap {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "MessageWrap({self})")
    }
}

#[derive(Clone, Eq, PartialEq, Getters, CopyGetters)]
pub struct Message {
    rust_message: RustMessage,

    /// Specifies which network this message is meant for.
    #[getset(get_copy = "pub")]
    network_id: u64,
    /// The [`MessageId`]s that this message directly approves.
    parents: Vec<MessageId>,
    /// The optional [Payload] of the message.
    payload: Option<MessagePayload>,
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
        write!(f, "Message({self})")
    }
}

impl From<RustMessage> for Message {
    fn from(message: RustMessage) -> Self {
        let payload: Option<MessagePayload> = message.payload().as_ref().map(|p| p.clone().into());
        Self {
            rust_message: message.clone(),
            network_id: message.network_id(),
            parents: message.parents().to_vec(),
            payload,
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
        self.id
    }

    pub fn parents(&self) -> Vec<MessageId> {
        self.parents.clone()
    }

    pub fn payload(&self) -> Option<MessagePayload> {
        self.payload.clone()
    }

    pub fn to_inner_clone(self) -> RustMessage {
        self.rust_message
    }

    pub fn deserialize(serialised_data: &str) -> Result<Self> {
        let res: Result<RustMessage, _> = serde_json::from_str(serialised_data);

        match res {
            Ok(s) => Ok(s.into()),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn serialize(&self) -> Result<String> {
        let res = serde_json::to_string(&self.rust_message);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }
}

pub struct MessageBuilder {
    builder: Rc<RefCell<Option<RustMessageBuilder<RustClientMiner>>>>,
}

impl Default for MessageBuilder {
    fn default() -> Self {
        Self {
            builder: Rc::new(RefCell::new(Option::from(
                RustMessageBuilder::<RustClientMiner>::default(),
            ))),
        }
    }
}

impl MessageBuilder {
    pub fn new() -> Self {
        MessageBuilder::new_with_builder(RustMessageBuilder::<RustClientMiner>::new())
    }

    fn new_with_builder(builder: RustMessageBuilder<RustClientMiner>) -> Self {
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
            .with_parents(Parents::new(parents).map_err(|e| anyhow::anyhow!(e.to_string()))?);
        Ok(MessageBuilder::new_with_builder(new_builder))
    }

    /// Adds a payload to a `MessageBuilder`.
    pub fn payload(&self, payload: MessagePayload) -> Self {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_payload(payload.to_inner());
        MessageBuilder::new_with_builder(new_builder)
    }

    /// Adds a nonce provider to a `MessageBuilder`.
    pub fn nonce_provider(&self, provider: ClientMiner, target_score: f64) -> Self {
        let new_builder = self
            .builder
            .borrow_mut()
            .take()
            .unwrap()
            .with_nonce_provider(provider.to_inner(), target_score);
        MessageBuilder::new_with_builder(new_builder)
    }

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
    pub fn with_seed(&self, seed: &str) -> Result<Self> {
        match &hex::decode(seed) {
            Ok(s) => {
                let mut fields = self.fields.borrow_mut().take().unwrap();
                fields.seed = Some(RustSeed::from_bytes(s));
                Ok(ClientMessageBuilder::new_with_fields(fields))
            }
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub(crate) fn with_seed_old(&self, seed: &str) -> Self {
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
        fields.builder = fields.builder.with_index(index);
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
        fields.builder = fields.builder.with_data(data);
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Set data to the builder
    pub fn with_data_string(&self, data: &str) -> Self {
        let mut fields = self.fields.borrow_mut().take().unwrap();
        fields.builder = fields.builder.with_data(data.to_string().as_bytes().to_vec());
        ClientMessageBuilder::new_with_fields(fields)
    }

    /// Prepare a transaction
    pub fn prepare_transaction(&self) -> Result<PreparedTransactionData> {
        let inner = self.fields.borrow_mut().take().unwrap();
        let res = crate::block_on(async {
            if let Some(s) = inner.seed {
                inner.builder.with_seed(&s).prepare_transaction().await
            } else {
                inner.builder.prepare_transaction().await
            }
        });
        match res {
            Ok(pt) => Ok(pt.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn sign_transaction(
        &self,
        prepared_transaction_data: PreparedTransactionData,
        seed: &str,
        inputs_range_low: usize,
        inputs_range_high: usize,
    ) -> Result<MessagePayload> {
        let second_seed = Some(RustSeed::from_bytes(
            &hex::decode(seed).map_err(|e| anyhow::anyhow!(e.to_string()))?,
        ));

        let mut range = None;
        if inputs_range_low != 0 {
            range = Some(inputs_range_low..inputs_range_high);
        }

        let prepared = RustPreparedTransactionData {
            essence: prepared_transaction_data.essence.to_inner(),
            address_index_recorders: prepared_transaction_data
                .address_index_recorders
                .iter()
                .map(|a| addres_into_rust_address_recorder(a.clone()))
                .collect(),
        };
        let inner = self.fields.borrow_mut().take().unwrap();
        let res = crate::block_on(async {
            if let Some(s) = inner.seed {
                inner
                    .builder
                    .with_seed(&s)
                    .sign_transaction(prepared, second_seed.as_ref(), range)
                    .await
            } else {
                inner
                    .builder
                    .sign_transaction(prepared, second_seed.as_ref(), range)
                    .await
            }
        });
        match res {
            Ok(p) => Ok(p.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
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

    pub fn finish_message(&self, payload: MessagePayload) -> Result<Message> {
        let inner = self.fields.borrow_mut().take().unwrap();
        let payload = Some(payload.to_inner());
        let res = crate::block_on(async {
            if let Some(s) = inner.seed {
                inner.builder.with_seed(&s).finish_message(payload).await
            } else {
                inner.builder.finish_message(payload).await
            }
        });
        match res {
            Ok(m) => Ok(m.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn finish_message_transaction(&self, payload: TransactionPayload) -> Result<Message> {
        let inner = self.fields.borrow_mut().take().unwrap();
        let payload = Some(Payload::Transaction(Box::new(payload.to_inner())));
        let res = crate::block_on(async {
            if let Some(s) = inner.seed {
                inner.builder.with_seed(&s).finish_message(payload).await
            } else {
                inner.builder.finish_message(payload).await
            }
        });
        match res {
            Ok(m) => Ok(m.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn finish_message_index(&self, payload: IndexationPayload) -> Result<Message> {
        let inner = self.fields.borrow_mut().take().unwrap();
        let payload = Some(Payload::Indexation(Box::new(payload.to_inner())));
        let res = crate::block_on(async {
            if let Some(s) = inner.seed {
                inner.builder.with_seed(&s).finish_message(payload).await
            } else {
                inner.builder.finish_message(payload).await
            }
        });
        match res {
            Ok(m) => Ok(m.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn finish_message_milestone(&self, payload: MilestonePayload) -> Result<Message> {
        let inner = self.fields.borrow_mut().take().unwrap();
        let payload = Some(Payload::Milestone(Box::new(payload.to_inner())));
        let res = crate::block_on(async {
            if let Some(s) = inner.seed {
                inner.builder.with_seed(&s).finish_message(payload).await
            } else {
                inner.builder.finish_message(payload).await
            }
        });
        match res {
            Ok(m) => Ok(m.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn finish_message_receipt(&self, payload: ReceiptPayload) -> Result<Message> {
        let inner = self.fields.borrow_mut().take().unwrap();
        let payload = Some(Payload::Receipt(Box::new(payload.to_inner())));
        let res = crate::block_on(async {
            if let Some(s) = inner.seed {
                inner.builder.with_seed(&s).finish_message(payload).await
            } else {
                inner.builder.finish_message(payload).await
            }
        });
        match res {
            Ok(m) => Ok(m.into()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn finish_message_treasury(&self, payload: TreasuryPayload) -> Result<Message> {
        let inner = self.fields.borrow_mut().take().unwrap();
        let payload = Some(Payload::TreasuryTransaction(Box::new(payload.to_inner())));
        let res = crate::block_on(async {
            if let Some(s) = inner.seed {
                inner.builder.with_seed(&s).finish_message(payload).await
            } else {
                inner.builder.finish_message(payload).await
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
        Self { client }
    }

    pub fn index_string(&self, index: &str) -> Result<Vec<MessageId>> {
        let res = crate::block_on(async {
            RustGetMessageBuilder::new(self.client.borrow())
                .index(index.to_string().as_bytes())
                .await
        });
        match res {
            Ok(r) => Ok(r.to_vec()),
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
            Ok(r) => Ok(r.to_vec()),
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
            Ok(r) => Ok(r.to_vec()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}
