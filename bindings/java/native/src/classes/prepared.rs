// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{CopyGetters, Getters};
use iota_client::api::{
    AddressIndexRecorder as RustAddressIndexRecorder, PreparedTransactionData as RustPreparedTransactionData,
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
    pub address_index_recorders: Vec<AddressIndexRecorder>,
}

impl PreparedTransactionData {
    pub fn deserialize(serialised_data: &str) -> Result<PreparedTransactionData> {
        let res = serde_json::from_str(serialised_data);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn essence(&self) -> Essence {
        self.essence.clone()
    }

    pub fn address_index_recorders(&self) -> Vec<AddressIndexRecorder> {
        self.address_index_recorders.to_vec()
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
            "essence={}, address_index_recorders=({:?})",
            self.essence, self.address_index_recorders
        )
    }
}

impl core::fmt::Debug for PreparedTransactionData {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "PreparedTransactionData({self})")
    }
}

impl From<RustPreparedTransactionData> for PreparedTransactionData {
    fn from(prepared: RustPreparedTransactionData) -> Self {
        Self {
            essence: prepared.essence.into(),
            address_index_recorders: prepared.address_index_recorders.iter().map(|rec| rec.into()).collect(),
        }
    }
}

/// Structure for sorting of UnlockBlocks
#[derive(Clone, Getters, CopyGetters, Serialize, Deserialize)]
pub struct AddressIndexRecorder {
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

impl AddressIndexRecorder {
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

pub(crate) fn addres_into_rust_address_recorder(recorder: AddressIndexRecorder) -> RustAddressIndexRecorder {
    RustAddressIndexRecorder {
        account_index: recorder.account_index(),
        input: recorder.input().to_inner_clone(),
        output: recorder.output().to_rust_output(),
        address_index: recorder.address_index(),
        chain: recorder.chain().to_rust_chain(),
        internal: recorder.internal(),
        bech32_address: recorder.bech32_address().clone(),
    }
}

impl core::fmt::Display for AddressIndexRecorder {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "account_index={}, input={}, output={}, address_index={:?}, internal={:?}, bech32_address={:?}",
            self.account_index, self.input, self.output, self.address_index, self.internal, self.bech32_address
        )
    }
}

impl core::fmt::Debug for AddressIndexRecorder {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "AddressIndexRecorder({self})")
    }
}

impl From<&RustAddressIndexRecorder> for AddressIndexRecorder {
    fn from(recorder: &RustAddressIndexRecorder) -> Self {
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
