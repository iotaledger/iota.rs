// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::bee_message::{
    output::Output,
    prelude::TreasuryTransactionPayload as RustTreasuryPayload,
};

pub struct TreasuryPayload {
    payload: RustTreasuryPayload,
}

impl From<RustTreasuryPayload> for TreasuryPayload {
    fn from(payload: RustTreasuryPayload) -> Self {
        Self { payload }
    }
}

impl TreasuryPayload {
    pub fn output(&self) -> u64 {
        if let Output::Treasury(payload) = self.payload.output() {
            return payload.amount();
        }
        unreachable!()
    }
}