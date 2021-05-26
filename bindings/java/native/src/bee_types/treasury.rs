// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::bee_message::{
    output::Output,
    prelude::TreasuryTransactionPayload as RustTreasuryPayload,
};
use std::{
    fmt::{Display, Formatter},
};

pub struct TreasuryPayload(RustTreasuryPayload);

impl From<RustTreasuryPayload> for TreasuryPayload {
    fn from(payload: RustTreasuryPayload) -> Self {
        Self(payload)
    }
}

impl TreasuryPayload {
    pub fn output(&self) -> u64 {
        if let Output::Treasury(payload) = self.0.output() {
            return payload.amount();
        }
        unreachable!()
    }
}

impl Display for TreasuryPayload {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}