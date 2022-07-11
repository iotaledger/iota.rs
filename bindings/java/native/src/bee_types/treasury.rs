// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use getset::{CopyGetters, Getters};
use iota_client::{
    bee_message::{
        input::Input as RustInput, output::Output as RustOutput,
        prelude::TreasuryTransactionPayload as RustTreasuryPayload,
    },
    bee_rest_api::types::responses::TreasuryResponse as RustTreasuryResponse,
};
use std::fmt::{Display, Formatter};

use crate::{
    bee_types::{TreasuryInput, TreasuryOutput},
    Result,
};

/// Response of GET /api/v1/treasury.
/// Returns all information about the treasury.
#[derive(Clone, Debug, Eq, PartialEq, Getters, CopyGetters)]
pub struct TreasuryResponse {
    #[getset(get = "pub")]
    pub milestone_id: String,
    #[getset(get_copy = "pub")]
    pub amount: u64,
}

impl From<RustTreasuryResponse> for TreasuryResponse {
    fn from(response: RustTreasuryResponse) -> Self {
        Self {
            milestone_id: response.milestone_id,
            amount: response.amount,
        }
    }
}

impl Display for TreasuryResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "(milestone_id={}, amount={})", self.milestone_id, self.amount)
    }
}

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

    pub fn to_inner(self) -> RustTreasuryPayload {
        self.0
    }

    pub fn output(&self) -> TreasuryOutput {
        if let RustOutput::Treasury(payload) = self.0.output() {
            return payload.clone().into();
        }
        unreachable!()
    }

    pub fn input(&self) -> TreasuryInput {
        if let RustInput::Treasury(payload) = self.0.input() {
            return (*payload).into();
        }
        unreachable!()
    }

    pub fn deserialize(serialised_data: &str) -> Result<TreasuryPayload> {
        let res = serde_json::from_str(serialised_data);

        match res {
            Ok(s) => Ok(TreasuryPayload(s)),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn serialize(&self) -> Result<String> {
        let res = serde_json::to_string(&self.0);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }
}

impl Display for TreasuryPayload {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}
