// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::bee_message::{
    output::Output as RustOutput,
    input::Input as RustInput,
    prelude::TreasuryTransactionPayload as RustTreasuryPayload,
};
use std::{
    fmt::{Display, Formatter},
};

use crate::bee_types::{
    TreasuryOutput,
    TreasuryInput,
};

pub struct TreasuryPayload(RustTreasuryPayload);

impl From<RustTreasuryPayload> for TreasuryPayload {
    fn from(payload: RustTreasuryPayload) -> Self {
        Self(payload)
    }
}

impl TreasuryPayload {
    pub fn new(input: TreasuryInput, output: TreasuryOutput) -> Self {
        Self(RustTreasuryPayload::new(input.to_inner_clone().into(), output.to_inner_clone().into()).unwrap())
    }

    pub fn output(&self) -> TreasuryOutput {
        if let RustOutput::Treasury(payload) = self.0.output() {
            return payload.clone().into()
        }
        unreachable!()
    }

    pub fn input(&self) -> TreasuryInput {
        if let RustInput::Treasury(payload) = self.0.input() {
            return payload.clone().into();
        }
        unreachable!()
    }
}

impl Display for TreasuryPayload {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}