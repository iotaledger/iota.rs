// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{CopyGetters, Getters};
use std::fmt::{Display, Formatter};

use crate::{
    SignatureLockedSingleOutput,
    classes::address::AddressDto
};

use iota_client::{
    bee_message::payload::{
        receipt::{
            ReceiptPayload as RustReceiptPayload,
            MigratedFundsEntry as RustMigratedFundsEntry,
        },
    },
    bee_rest_api::types::dtos::{
        MigratedFundsEntryDto as RustMigratedFundsEntryDto, PayloadDto as RustPayloadDto, ReceiptDto as RustReceiptDto,
        ReceiptPayloadDto as RustReceiptPayloadDto,
    },
};

#[derive(Getters, CopyGetters, Debug)]
pub struct ReceiptDto {
    pub receipt: ReceiptPayloadDto,
    #[getset(get_copy = "pub")]
    pub milestone_index: u32,
}

impl From<RustReceiptDto> for ReceiptDto {
    fn from(receipt: RustReceiptDto) -> Self {
        Self {
            receipt: receipt.receipt.into(),
            milestone_index: receipt.milestone_index.clone(),
        }
    }
}

impl Display for ReceiptDto {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}: {})", self.milestone_index, self.receipt.to_string())
    }
}

#[derive(Getters, CopyGetters, Debug)]
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
            kind: payload.kind.clone(),
            migrated_at: payload.migrated_at.clone(),
            funds: payload.funds.into_iter().map(|m| m.into()).collect(),
            transaction: payload.transaction.into(),
            last: payload.last.clone(),
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters, PartialEq)]
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
        write!(f, "(tail_transaction_hash={:}, address={:?}, deposit={})", self.tail_transaction_hash, self.address, self.deposit)
    }
}

pub struct ReceiptPayload {
    payload: RustReceiptPayload,
}

impl From<RustReceiptPayload> for ReceiptPayload {
    fn from(payload: RustReceiptPayload) -> Self {
        Self { payload }
    }
}

impl ReceiptPayload {
    pub fn migrated_at(&self) -> u32 {
        *self.payload.migrated_at()
    }

    pub fn last(&self) -> bool {
        self.payload.last()
    }

    pub fn funds(&self) -> Vec<MigratedFundsEntry> {
        self.payload
            .funds()
            .into_iter()
            .map(|m| MigratedFundsEntry { payload: m.clone() })
            .collect()
    }
}

impl Display for ReceiptPayload {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.payload)
    }
}

pub struct MigratedFundsEntry {
    payload: RustMigratedFundsEntry,
}

impl MigratedFundsEntry {
    pub fn tail_transaction_hash(&self) -> Vec<u8> {
        self.payload.tail_transaction_hash().as_ref().to_vec()
    }

    pub fn output(&self) -> SignatureLockedSingleOutput {
        self.payload.output().clone().into()
    }
}

impl Display for MigratedFundsEntry {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.payload)
    }
}