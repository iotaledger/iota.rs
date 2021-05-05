// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
/*
use iota_wallet::{
    address::OutputKind as RustOutputKind,
    message::{
        MessageTransactionPayload as MessageTransactionPayloadRust, TransactionEssence as TransactionEssenceRust,
        TransactionInput as RustWalletInput, TransactionOutput as RustWalletOutput,
        TransactionRegularEssence as TransactionRegularEssenceRust,
    },
};

use iota::{Payload as RustPayload, UnlockBlock as RustUnlockBlock};

pub enum InputKind {
    Utxo = 0,
    Treasury = 1,
}

pub enum UnlockBlockKind {
    Reference = 0,
    Ed25519 = 1,
}

pub struct MessageTransactionPayload {
    essence: Essence,
    unlock_blocks: Vec<UnlockBlock>,
}

impl From<&Box<MessageTransactionPayloadRust>> for MessageTransactionPayload {
    fn from(payload: &Box<MessageTransactionPayloadRust>) -> Self {
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

impl MessageTransactionPayload {
    pub fn essence(&self) -> Essence {
        self.essence.clone()
    }

    pub fn unlock_blocks(&self) -> Vec<UnlockBlock> {
        self.unlock_blocks.iter().cloned().collect()
    }
}
#[derive(Clone)]
pub struct Essence {
    essence: TransactionEssenceRust,
}

impl Essence {
    #[allow(irrefutable_let_patterns)]
    pub fn get_as_regular(&self) -> Option<RegularEssence> {
        if let TransactionEssenceRust::Regular(essence) = &self.essence {
            return Some(RegularEssence {
                essence: essence.clone(),
            });
        };
        None
    }
}

#[derive(Clone)]
pub struct RegularEssence {
    essence: TransactionRegularEssenceRust,
}

impl RegularEssence {
    pub fn inputs(&self) -> Vec<TransactionInput> {
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
    }

    /// Whether the transaction is between the mnemonic accounts or not.
    pub fn internal(&self) -> bool {
        self.essence.internal()
    }

    /// Whether the transaction is incoming or outgoing.
    pub fn incoming(&self) -> bool {
        self.essence.incoming()
    }

    /// The transactions's value.
    pub fn value(&self) -> u64 {
        self.essence.value()
    }

    /// The transactions's remainder value sum.
    pub fn remainder_value(&self) -> u64 {
        self.essence.remainder_value()
    }
}

#[derive(Clone)]
pub struct TransactionInput {
    input: RustWalletInput,
}

impl TransactionInput {
    pub fn kind(&self) -> InputKind {
        match self.input {
            RustWalletInput::Utxo(_) => InputKind::Utxo,
            RustWalletInput::Treasury(_) => InputKind::Treasury,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.input)
    }
}

#[derive(Clone)]
pub struct TransactionOutput {
    output: RustWalletOutput,
}

impl TransactionOutput {
    pub fn kind(&self) -> RustOutputKind {
        match self.output {
            RustWalletOutput::SignatureLockedSingle(_) => RustOutputKind::SignatureLockedSingle,
            RustWalletOutput::SignatureLockedDustAllowance(_) => RustOutputKind::SignatureLockedDustAllowance,
            RustWalletOutput::Treasury(_) => RustOutputKind::Treasury,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.output)
    }
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
*/