// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::fmt::{
    Formatter, Display,
};

use iota_client::bee_message::prelude::{
    UtxoInput as RustUtxoInput,
};

pub struct UtxoInput(RustUtxoInput);

impl UtxoInput {
    /// Returns the `TransactionId` of an `OutputId`.
    pub fn transaction_id(&self) -> Vec<u8> {
        self.0.output_id().transaction_id().as_ref().to_vec()
    }

    /// Returns the index of an `OutputId`.
    pub fn index(&self) -> u16 {
        self.0.output_id().index()
    }
}
impl Display for UtxoInput {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "(transaction_id={}, index={})", hex::encode(self.transaction_id()), self.index())
    }
}

impl From<RustUtxoInput> for UtxoInput {
    fn from(input: RustUtxoInput) -> Self {
        Self(input)
    }
}