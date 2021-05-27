// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::fmt::{Display, Formatter};

use iota_client::bee_message::{
    input::Input as RustInput,
    payload::{
        milestone::MilestoneId,
        transaction::TransactionId,
    },
    prelude::UtxoInput as RustUtxoInput,
    prelude::TreasuryInput as RustTreasuryInput,
};

use crate::Result;

pub enum InputKind {
    Utxo = 0,
    Treasury = 1,
}

#[derive(Clone)]
pub struct Input(RustInput);

impl Input {
    pub fn kind(&self) -> InputKind {
        match self.0 {
            RustInput::Utxo(_) => InputKind::Utxo,
            RustInput::Treasury(_) => InputKind::Treasury,
        }
    }

    pub fn get_as_utxo(&self) -> Option<UtxoInput> {
        if let RustInput::Utxo(payload) = &self.0 {
            Some(payload.clone().into())
        } else {
            None
        }
    }

    pub fn get_as_treasury(&self) -> Option<TreasuryInput> {
        if let RustInput::Treasury(payload) = self.0 {
            Some(payload.into())
        } else {
            None
        }
    }
}

impl From<RustInput> for Input {
    fn from(input: RustInput) -> Self {
        Self(input)
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

pub struct UtxoInput(RustUtxoInput);

impl UtxoInput {
    pub fn from(id: TransactionId, index: u16) -> Result<Self> {
        match RustUtxoInput::new(id, index) {
            Ok(e) => Ok(Self(e)),
            Err(e) => Err(anyhow::anyhow!(e.to_string()))
        }
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

pub struct TreasuryInput(RustTreasuryInput);

impl TreasuryInput {
    pub fn new(id: MilestoneId) -> Self {
        Self(RustTreasuryInput::new(id))
    }

    pub fn milestone_id(&self) -> MilestoneId {
        *self.0.milestone_id()
    }

    pub fn to_inner_clone(&self) -> RustTreasuryInput {
        self.0.clone()
    }
}

impl From<RustTreasuryInput> for TreasuryInput {
    fn from(input: RustTreasuryInput) -> Self {
        Self(input)
    }
}

impl Display for TreasuryInput {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}