// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    Result,
    bee_types::{
        Input,
        OutputDto,
    },
};

use anyhow::anyhow;

use std::{cell::RefCell, rc::Rc, fmt::{Display, Formatter}};

use iota_client::bee_message::{
    unlock::{
        UnlockBlock as RustUnlockBlock,
        UnlockBlocks as RustUnlockBlocks,
    },
    payload::{
        transaction::{
            TransactionId,
            TransactionPayload as RustTransactionPayload,
            TransactionPayloadBuilder as RustTransactionPayloadBuilder,
            Essence as RustEssence, 
            RegularEssence as RustRegularEssence,
        },
    },
};

pub enum UnlockBlockKind {
    Reference = 0,
    Ed25519 = 1,
}

pub struct TransactionPayload {
    essence: Essence,
    unlock_blocks: Vec<UnlockBlock>,
    id: TransactionId,
}

impl From<RustTransactionPayload> for TransactionPayload {
    fn from(payload: RustTransactionPayload) -> Self {
        Self {
            essence: Essence(payload.essence().to_owned()),
            unlock_blocks: payload
                .unlock_blocks()
                .iter()
                .cloned()
                .map(|unlock_block| UnlockBlock(unlock_block))
                .collect(),
            id: payload.id()
        }
    }
}

impl TransactionPayload {
    pub fn builder() -> TransactionPayloadBuilder {
        TransactionPayloadBuilder::new()
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
        write!(f, "(id={}, essence={}, unlock_blocks=({:?}))", self.id, self.essence, self.unlock_blocks)
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
        self.0
            .inputs()
            .iter()
            .cloned()
            .map(|input| input.into())
            .collect()
    }

    /// Gets the transaction outputs.
    pub fn outputs(&self) -> Vec<OutputDto> {
        self.0
            .outputs()
            .iter()
            .map(|output| output.into())
            .collect()
    }
/*
    /// Gets the transaction chained payload.
    pub fn payload(&self) -> &Option<RustPayload> {
        self.essence.payload()
    }*/
}

impl Display for RegularEssence {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct UnlockBlock(RustUnlockBlock);

impl UnlockBlock {
    pub fn kind(&self) -> UnlockBlockKind {
        match self.0 {
            RustUnlockBlock::Signature(_) => UnlockBlockKind::Ed25519,
            RustUnlockBlock::Reference(_) => UnlockBlockKind::Reference,
            _ => panic!("Found unknown unlock block"),
        }
    }

    pub fn to_inner(&self) -> RustUnlockBlock   {
        self.0.clone()
    }
}
impl Display for UnlockBlock {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct UnlockBlocks(RustUnlockBlocks);

impl UnlockBlocks {

    pub fn new(unlock_blocks: Vec<UnlockBlock>) -> Result<Self> {
        match RustUnlockBlocks::new(unlock_blocks.iter().map(|b| b.to_inner()).collect()) {
            Err(e) => Err(anyhow!(e.to_string())),
            Ok(u) => Ok(UnlockBlocks(u))
        }
    }

    pub fn get(&self, index: usize) -> Option<UnlockBlock> {
        match self.0.get(index) {
            None => None,
            Some(u) => Some(UnlockBlock(u.clone()))
        }
    }

    pub fn to_inner(&self) -> RustUnlockBlocks   {
        self.0.clone()
    }
}

impl Display for UnlockBlocks {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

pub struct TransactionPayloadBuilder(Rc<RefCell<Option<RustTransactionPayloadBuilder>>>);

impl TransactionPayloadBuilder{
    /// Creates a new `TransactionPayloadBuilder`.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(Option::from(RustTransactionPayloadBuilder::default()))))
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
        let new_builder = self.0.borrow_mut().take().unwrap().with_unlock_blocks(unlock_blocks.to_inner());
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