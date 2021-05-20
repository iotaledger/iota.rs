// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use anyhow::Error;
use getset::{CopyGetters, Getters};
use std::convert::TryFrom;

use iota_client::bee_rest_api::types::{
    dtos::OutputDto as RustOutputDto, responses::OutputResponse as RustOutputResponse,
};

use crate::classes::address::AddressDto;

#[derive(Getters, CopyGetters)]
pub struct OutputResponse {
    #[getset(get = "pub")]
    pub message_id: String,
    #[getset(get = "pub")]
    pub transaction_id: String,
    #[getset(get_copy = "pub")]
    pub output_index: u16,
    #[getset(get_copy = "pub")]
    pub is_spent: bool,
    pub output: OutputDto,
}

impl OutputResponse {
    pub fn output(&self) -> OutputDto {
        self.output.clone()
    }
}

impl From<RustOutputResponse> for OutputResponse {
    fn from(output: RustOutputResponse) -> Self {
        Self {
            message_id: output.message_id.clone(),
            transaction_id: output.transaction_id.clone(),
            output_index: output.output_index,
            is_spent: output.is_spent,
            output: OutputDto {
                output: output.output.clone(),
            },
        }
    }
}

pub enum OutputKind {
    SignatureLockedSingle = 1,
    SignatureLockedDustAllowance = 2,
    Treasury = 3,
}

#[derive(Clone)]
pub struct OutputDto {
    output: RustOutputDto,
}

impl OutputDto {
    pub fn to_string(&self) -> String {
        format!("{:?}", self.output)
    }

    pub fn kind(&self) -> OutputKind {
        match self.output {
            RustOutputDto::SignatureLockedSingle(_) => OutputKind::SignatureLockedSingle,
            RustOutputDto::SignatureLockedDustAllowance(_) => OutputKind::SignatureLockedDustAllowance,
            RustOutputDto::Treasury(_) => OutputKind::Treasury,
        }
    }

    pub fn as_signature_locked_single_output_dto(&self) -> anyhow::Result<SignatureLockedSingleOutputDto> {
        SignatureLockedSingleOutputDto::try_from(&self.output)
    }

    pub fn as_signature_locked_dust_allowance_output_dto(
        &self,
    ) -> anyhow::Result<SignatureLockedDustAllowanceOutputDto> {
        SignatureLockedDustAllowanceOutputDto::try_from(&self.output)
    }

    pub fn as_treasury_output(&self) -> anyhow::Result<TreasuryOutputDto> {
        TreasuryOutputDto::try_from(&self.output)
    }
}

impl TryFrom<&RustOutputDto> for SignatureLockedSingleOutputDto {
    type Error = Error;
    fn try_from(output: &RustOutputDto) -> Result<Self, Self::Error> {
        match output {
            RustOutputDto::SignatureLockedSingle(ed) => Ok(Self {
                kind: ed.kind,
                address: ed.address.clone().into(),
                amount: ed.amount,
            }),
            _ => unimplemented!(),
        }
    }
}

/// Describes a deposit to a single address which is unlocked via a signature.
#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct SignatureLockedSingleOutputDto {
    #[getset(get_copy = "pub")]
    pub kind: u8,
    #[getset(get = "pub")]
    pub address: AddressDto,
    #[getset(get_copy = "pub")]
    pub amount: u64,
}

impl TryFrom<&RustOutputDto> for SignatureLockedDustAllowanceOutputDto {
    type Error = Error;
    fn try_from(output: &RustOutputDto) -> Result<Self, Self::Error> {
        match output {
            RustOutputDto::SignatureLockedDustAllowance(ed) => Ok(Self {
                kind: ed.kind,
                address: ed.address.clone().into(),
                amount: ed.amount,
            }),
            _ => unimplemented!(),
        }
    }
}

/// Output type for deposits that enables an address to receive dust outputs. It can be consumed as an input like a
/// regular SigLockedSingleOutput.
#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct SignatureLockedDustAllowanceOutputDto {
    #[getset(get_copy = "pub")]
    pub kind: u8,
    #[getset(get = "pub")]
    pub address: AddressDto,
    #[getset(get_copy = "pub")]
    pub amount: u64,
}

impl TryFrom<&RustOutputDto> for TreasuryOutputDto {
    type Error = Error;
    fn try_from(output: &RustOutputDto) -> Result<Self, Self::Error> {
        match output {
            RustOutputDto::Treasury(ed) => Ok(Self {
                kind: ed.kind,
                amount: ed.amount,
            }),
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, Getters, CopyGetters)]
pub struct TreasuryOutputDto {
    #[getset(get_copy = "pub")]
    pub kind: u8,
    #[getset(get_copy = "pub")]
    pub amount: u64,
}
