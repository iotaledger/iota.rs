// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    bee_types::InputKind,
};

use iota_client::bee_message::{
    unlock::UnlockBlock as RustUnlockBlock,
    payload::{
        transaction::{
            TransactionPayload as RustTransactionPayload,
            Essence as RustEssence, 
            RegularEssence as RustRegularEssence,
        }
    },
};

pub enum UnlockBlockKind {
    Reference = 0,
    Ed25519 = 1,
}

pub struct TransactionPayload {
    essence: Essence,
    unlock_blocks: Vec<UnlockBlock>,
}

impl From<&Box<RustTransactionPayload>> for TransactionPayload {
    fn from(payload: &Box<RustTransactionPayload>) -> Self {
        Self {
            essence: Essence {
                essence: payload.essence().to_owned(),
            },
            unlock_blocks: payload
                .unlock_blocks()
                .iter()
                .cloned()
                .map(|unlock_block| UnlockBlock {
                    unlock_block: unlock_block,
                })
                .collect(),
        }
    }
}

impl TransactionPayload {
    pub fn essence(&self) -> Essence {
        self.essence.clone()
    }

    pub fn unlock_blocks(&self) -> Vec<UnlockBlock> {
        self.unlock_blocks.iter().cloned().collect()
    }
}

#[derive(Clone)]
pub struct Essence {
    essence: RustEssence,
}

impl Essence {
    #[allow(irrefutable_let_patterns)]
    pub fn get_as_regular(&self) -> Option<RegularEssence> {
        if let RustEssence::Regular(essence) = &self.essence {
            return Some(RegularEssence {
                essence: essence.clone(),
            });
        };
        None
    }
}

#[derive(Clone)]
pub struct RegularEssence {
    essence: RustRegularEssence,
}

impl RegularEssence {
    /*pub fn inputs(&self) -> Vec<TransactionInput> {
        self.essence
            .inputs()
            .iter()
            .cloned()
            .map(|input| TransactionInput { input: input })
            .collect()
    }

    /// Gets the transaction outputs.
    pub fn outputs(&self) -> Vec<TransactionOutput> {
        self.essence
            .outputs()
            .iter()
            .cloned()
            .map(|output| TransactionOutput { output: output })
            .collect()
    }

    /// Gets the transaction chained payload.
    pub fn payload(&self) -> &Option<RustPayload> {
        self.essence.payload()
    }*/
}

#[derive(Clone)]
pub struct UnlockBlock {
    unlock_block: RustUnlockBlock,
}

impl UnlockBlock {
    pub fn kind(&self) -> UnlockBlockKind {
        match self.unlock_block {
            RustUnlockBlock::Signature(_) => UnlockBlockKind::Ed25519,
            RustUnlockBlock::Reference(_) => UnlockBlockKind::Reference,
            _ => panic!("Found unknown unlock block"),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.unlock_block)
    }
}