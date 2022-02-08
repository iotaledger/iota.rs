// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{CopyGetters, Getters};
use iota_client::api::{
    InputSigningData as RustInputSigningData, PreparedTransactionData as RustPreparedTransactionData,
};
use serde::{Deserialize, Serialize};

use crate::{
    bee_types::{Essence, Input, OutputResponse},
    slip10::*,
    Result,
};

/// Helper struct for offline signing
#[derive(Clone, Getters, CopyGetters, Serialize, Deserialize)]
pub struct PreparedTransactionData {
    /// Transaction essence
    pub essence: Essence,
    /// Required address information for signing
    pub input_signing_data_entrys: Vec<InputSigningData>,
}

impl PreparedTransactionData {
    pub fn deserialize(serialised_data: &str) -> Result<PreparedTransactionData> {
        let res = serde_json::from_str(&serialised_data);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn essence(&self) -> Essence {
        self.essence.clone()
    }

    pub fn input_signing_data_entrys(&self) -> Vec<InputSigningData> {
        self.input_signing_data_entrys.iter().cloned().collect()
    }

    pub fn serialize(&self) -> Result<String> {
        let res = serde_json::to_string(self);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }
}

impl core::fmt::Display for PreparedTransactionData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "essence={}, input_signing_data_entrys=({:?})",
            self.essence, self.input_signing_data_entrys
        )
    }
}

impl core::fmt::Debug for PreparedTransactionData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "PreparedTransactionData({})", self)
    }
}

impl From<RustPreparedTransactionData> for PreparedTransactionData {
    fn from(prepared: RustPreparedTransactionData) -> Self {
        Self {
            essence: prepared.essence.into(),
            input_signing_data_entrys: prepared.input_signing_data_entrys.iter().map(|rec| rec.into()).collect(),
        }
    }
}

/// Structure for sorting of UnlockBlocks
#[derive(Clone, Getters, CopyGetters, Serialize, Deserialize)]
pub struct InputSigningData {
    #[getset(get_copy = "pub")]
    account_index: usize,
    input: Input,
    output: OutputResponse,
    #[getset(get_copy = "pub")]
    address_index: usize,
    chain: Chain,
    #[getset(get_copy = "pub")]
    internal: bool,
    #[getset(get = "pub")]
    bech32_address: String,
}

impl InputSigningData {
    pub fn input(&self) -> Input {
        self.input.clone()
    }

    pub fn output(&self) -> OutputResponse {
        self.output.clone()
    }

    pub fn chain(&self) -> Chain {
        self.chain.clone()
    }
}

pub(crate) fn addres_into_rust_address_recorder(recorder: InputSigningData) -> RustInputSigningData {
    RustInputSigningData {
        account_index: recorder.account_index(),
        input: recorder.input().to_inner_clone(),
        output: recorder.output().to_rust_output(),
        address_index: recorder.address_index(),
        chain: recorder.chain().to_rust_chain(),
        internal: recorder.internal(),
        bech32_address: recorder.bech32_address().clone(),
    }
}

impl core::fmt::Display for InputSigningData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "account_index={}, input={}, output={}, address_index={:?}, internal={:?}, bech32_address={:?}",
            self.account_index, self.input, self.output, self.address_index, self.internal, self.bech32_address
        )
    }
}

impl core::fmt::Debug for InputSigningData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "InputSigningData({})", self)
    }
}

impl From<&RustInputSigningData> for InputSigningData {
    fn from(recorder: &RustInputSigningData) -> Self {
        Self {
            account_index: recorder.account_index,
            input: recorder.input.clone().into(),
            output: recorder.output.clone().into(),
            address_index: recorder.address_index,
            chain: recorder.chain.clone().into(),
            internal: recorder.internal,
            bech32_address: recorder.bech32_address.clone(),
        }
    }
}
