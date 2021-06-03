// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    bee_types::{Input, MessagePayload, OutputDto, UnlockBlock, UnlockBlocks},
    Result,
};

use anyhow::anyhow;

use std::{
    cell::RefCell,
    fmt::{Display, Formatter},
    rc::Rc,
};

use iota_client::bee_message::payload::transaction::{
    Essence as RustEssence, RegularEssence as RustRegularEssence, TransactionId,
    TransactionPayload as RustTransactionPayload, TransactionPayloadBuilder as RustTransactionPayloadBuilder,
};

pub struct TransactionPayload {
    rust_payload: RustTransactionPayload,
    essence: Essence,
    unlock_blocks: Vec<UnlockBlock>,
    id: TransactionId,
}

impl From<RustTransactionPayload> for TransactionPayload {
    fn from(payload: RustTransactionPayload) -> Self {
        Self {
            rust_payload: payload.clone(),
            essence: Essence(payload.essence().to_owned()),
            unlock_blocks: payload
                .unlock_blocks()
                .iter()
                .cloned()
                .map(|unlock_block| unlock_block.into())
                .collect(),
            id: payload.id(),
        }
    }
}

impl TransactionPayload {
    pub fn builder() -> TransactionPayloadBuilder {
        TransactionPayloadBuilder::new()
    }

    pub fn to_inner(self) -> RustTransactionPayload {
        self.rust_payload
    }
    pub fn essence(&self) -> Essence {
        self.essence.clone()
    }

    pub fn unlock_blocks(&self) -> Vec<UnlockBlock> {
        self.unlock_blocks.iter().cloned().collect()
    }

    pub fn id(&self) -> TransactionId {
        self.id.clone()
    }
}

impl Display for TransactionPayload {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "(id={}, essence={}, unlock_blocks=({:?}))",
            self.id, self.essence, self.unlock_blocks
        )
    }
}

#[derive(Clone)]
pub struct Essence(RustEssence);

impl Essence {
    #[allow(irrefutable_let_patterns)]
    pub fn get_as_regular(&self) -> Option<RegularEssence> {
        if let RustEssence::Regular(essence) = &self.0 {
            return Some(RegularEssence(essence.clone()));
        };
        None
    }

    pub fn to_inner(&self) -> RustEssence {
        self.0.clone()
    }
}

impl Display for Essence {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

#[derive(Clone)]
pub struct RegularEssence(RustRegularEssence);

impl RegularEssence {
    pub fn inputs(&self) -> Vec<Input> {
        self.0.inputs().iter().cloned().map(|input| input.into()).collect()
    }

    /// Gets the transaction outputs.
    pub fn outputs(&self) -> Vec<OutputDto> {
        self.0.outputs().iter().map(|output| output.into()).collect()
    }
    // Gets the transaction chained payload.
    pub fn payload(&self) -> Option<MessagePayload> {
        match self.0.payload() {
            Some(payload) => Some(payload.clone().into()),
            None => None,
        }
    }
}

impl Display for RegularEssence {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

pub struct TransactionPayloadBuilder(Rc<RefCell<Option<RustTransactionPayloadBuilder>>>);

impl TransactionPayloadBuilder {
    /// Creates a new `TransactionPayloadBuilder`.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Option::from(
            RustTransactionPayloadBuilder::default(),
        ))))
    }

    fn new_with_builder(builder: RustTransactionPayloadBuilder) -> Self {
        Self(Rc::new(RefCell::new(Option::from(builder))))
    }

    /// Adds an essence to a `TransactionPayloadBuilder`.
    pub fn with_essence(&self, essence: Essence) -> Self {
        let new_builder = self.0.borrow_mut().take().unwrap().with_essence(essence.to_inner());
        TransactionPayloadBuilder::new_with_builder(new_builder)
    }

    /// Adds unlock blocks to a `TransactionPayloadBuilder`.
    pub fn with_unlock_blocks(&self, unlock_blocks: UnlockBlocks) -> Self {
        let new_builder = self
            .0
            .borrow_mut()
            .take()
            .unwrap()
            .with_unlock_blocks(unlock_blocks.to_inner());
        TransactionPayloadBuilder::new_with_builder(new_builder)
    }

    /// Finishes a `TransactionPayloadBuilder` into a `TransactionPayload`.
    pub fn finish(&self) -> Result<TransactionPayload> {
        let res = self.0.borrow_mut().take().unwrap().finish();

        match res {
            Err(e) => Err(anyhow!(e.to_string())),
            Ok(p) => Ok(p.into()),
        }
    }
}
