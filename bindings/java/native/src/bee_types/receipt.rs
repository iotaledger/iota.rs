// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{CopyGetters, Getters};
use std::{
    convert::TryInto,
    fmt::{Display, Formatter},
};

use crate::{
    bee_types::{MessagePayload, TreasuryPayload},
    classes::address::AddressDto,
    Result, SignatureLockedSingleOutput,
};

use iota_client::{
    bee_message::{
        milestone::MilestoneIndex,
        payload::receipt::{
            MigratedFundsEntry as RustMigratedFundsEntry, ReceiptPayload as RustReceiptPayload, TailTransactionHash,
        },
    },
    bee_rest_api::types::dtos::{
        MigratedFundsEntryDto as RustMigratedFundsEntryDto, PayloadDto as RustPayloadDto, ReceiptDto as RustReceiptDto,
        ReceiptPayloadDto as RustReceiptPayloadDto,
    },
};

#[derive(Getters, CopyGetters, Debug)]
pub struct ReceiptDto {
    receipt: ReceiptPayloadDto,
    #[getset(get_copy = "pub")]
    pub milestone_index: u32,
}

impl ReceiptDto {
    pub fn receipt(&self) -> ReceiptPayloadDto {
        self.receipt.clone()
    }
}

impl From<RustReceiptDto> for ReceiptDto {
    fn from(receipt: RustReceiptDto) -> Self {
        Self {
            receipt: receipt.receipt.into(),
            milestone_index: receipt.milestone_index,
        }
    }
}

impl Display for ReceiptDto {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}: {})", self.milestone_index, self.receipt)
    }
}

#[derive(Getters, CopyGetters, Debug, Clone)]
pub struct ReceiptPayloadDto {
    #[getset(get_copy = "pub")]
    pub kind: u32,
    #[getset(get_copy = "pub")]
    pub migrated_at: u32,
    pub funds: Vec<MigratedFundsEntryDto>,

    // Actual payload
    pub transaction: RustPayloadDto,

    #[getset(get_copy = "pub")]
    pub last: bool,
}

impl ReceiptPayloadDto {
    pub fn funds(&self) -> Vec<MigratedFundsEntryDto> {
        self.funds.clone()
    }
}

impl Display for ReceiptPayloadDto {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "(kind: {}, migrated at: {}, last: {}, funds: {:?})",
            self.kind, self.migrated_at, self.last, self.funds
        )
    }
}

impl From<RustReceiptPayloadDto> for ReceiptPayloadDto {
    fn from(payload: RustReceiptPayloadDto) -> Self {
        Self {
            kind: payload.kind,
            migrated_at: payload.migrated_at,
            funds: payload.funds.into_iter().map(|m| m.into()).collect(),
            transaction: payload.transaction,
            last: payload.last,
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters, Eq, PartialEq)]
pub struct MigratedFundsEntryDto {
    #[getset(get = "pub")]
    pub tail_transaction_hash: String,
    pub address: AddressDto,
    #[getset(get_copy = "pub")]
    pub deposit: u64,
}

impl MigratedFundsEntryDto {
    pub fn address(&self) -> AddressDto {
        self.address.clone()
    }
}

impl From<RustMigratedFundsEntryDto> for MigratedFundsEntryDto {
    fn from(value: RustMigratedFundsEntryDto) -> Self {
        MigratedFundsEntryDto {
            tail_transaction_hash: value.tail_transaction_hash,
            address: value.address.into(),
            deposit: value.deposit,
        }
    }
}
impl Display for MigratedFundsEntryDto {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "(tail_transaction_hash={:}, address={:?}, deposit={})",
            self.tail_transaction_hash, self.address, self.deposit
        )
    }
}

pub struct ReceiptPayload(RustReceiptPayload);

impl From<RustReceiptPayload> for ReceiptPayload {
    fn from(payload: RustReceiptPayload) -> Self {
        Self(payload)
    }
}

impl ReceiptPayload {
    pub fn to_inner(self) -> RustReceiptPayload {
        self.0
    }
    pub fn from(
        migrated_at: u32,
        last: bool,
        funds: Vec<MigratedFundsEntry>,
        transaction: MessagePayload,
    ) -> Result<Self> {
        let res = RustReceiptPayload::new(
            MilestoneIndex::new(migrated_at),
            last,
            funds.iter().map(|f| f.to_inner()).collect(),
            transaction.to_inner(),
        );
        match res {
            Ok(payload) => Ok(Self(payload)),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn migrated_at(&self) -> u32 {
        *self.0.migrated_at()
    }

    pub fn last(&self) -> bool {
        self.0.last()
    }

    pub fn transaction(&self) -> TreasuryPayload {
        let p: MessagePayload = self.0.transaction().clone().into();
        p.as_treasury().unwrap()
    }

    pub fn amount(&self) -> u64 {
        self.0.amount()
    }

    pub fn funds(&self) -> Vec<MigratedFundsEntry> {
        self.0
            .funds()
            .iter()
            .map(|m| MigratedFundsEntry { payload: m.clone() })
            .collect()
    }

    pub fn deserialize(serialised_data: &str) -> Result<ReceiptPayload> {
        let res = serde_json::from_str(serialised_data);

        match res {
            Ok(s) => Ok(ReceiptPayload(s)),
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

impl Display for ReceiptPayload {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct MigratedFundsEntry {
    payload: RustMigratedFundsEntry,
}

impl MigratedFundsEntry {
    pub fn from(hash: String, output: SignatureLockedSingleOutput) -> Result<MigratedFundsEntry> {
        let tail_res = TailTransactionHash::new(hash.as_bytes().try_into().unwrap());
        match tail_res {
            Ok(tail) => {
                let res = RustMigratedFundsEntry::new(tail, output.to_inner_clone());
                match res {
                    Ok(payload) => Ok(Self { payload }),
                    Err(e) => Err(anyhow::anyhow!(e.to_string())),
                }
            }
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn tail_transaction_hash(&self) -> String {
        self.payload.tail_transaction_hash().to_string()
    }

    pub fn output(&self) -> SignatureLockedSingleOutput {
        self.payload.output().clone().into()
    }

    fn to_inner(&self) -> RustMigratedFundsEntry {
        self.payload.clone()
    }
}

impl Display for MigratedFundsEntry {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.payload)
    }
}
