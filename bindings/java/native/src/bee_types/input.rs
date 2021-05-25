// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::fmt::{Display, Formatter};

use iota_client::bee_message::{
    input::Input as RustInput,
    payload::transaction::TransactionId,
    prelude::UtxoInput as RustUtxoInput,
};

use crate::Result;

pub enum InputKind {
    Utxo = 0,
    Treasury = 1,
}

#[derive(Clone)]
pub struct TransactionInput {
    input: RustInput,
}

impl TransactionInput {
    pub fn kind(&self) -> InputKind {
        match self.input {
            RustInput::Utxo(_) => InputKind::Utxo,
            RustInput::Treasury(_) => InputKind::Treasury,
            _ => unimplemented!()
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self.input)
    }
}

pub struct UtxoInput(RustUtxoInput);

impl UtxoInput {
    pub fn from(id: TransactionId, index: u16) -> Result<Self> {
        Ok(Self(RustUtxoInput::new(id, index)?))
    }

    /// Returns the `TransactionId` of an `OutputId`.
    pub fn transaction_id(&self) -> Vec<u8> {
        self.0.output_id().transaction_id().as_ref().to_vec()
    }

    /// Returns the index of an `OutputId`.
    pub fn index(&self) -> u16 {
        self.0.output_id().index()
    }

    pub fn to_inner_clone(&self) -> RustUtxoInput {
        self.0.clone()
    }
}
impl Display for UtxoInput {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "(transaction_id={}, index={})",
            hex::encode(self.transaction_id()),
            self.index()
        )
    }
}

impl From<RustUtxoInput> for UtxoInput {
    fn from(input: RustUtxoInput) -> Self {
        Self(input)
    }
}
