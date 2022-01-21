// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use anyhow::Error;
use getset::{CopyGetters, Getters};
use serde::{Deserialize, Serialize};
use std::{
    convert::TryFrom,
    fmt::{Display, Formatter},
};

use iota_client::{
    bee_message::output::{
        Output as RustOutput, SignatureLockedDustAllowanceOutput as RustSignatureLockedDustAllowanceOutput,
        SignatureLockedSingleOutput as RustSignatureLockedSingleOutput, TreasuryOutput as RustTreasuryOutput,
    },
    bee_rest_api::types::{dtos::OutputDto as RustOutputDto, responses::OutputResponse as RustOutputResponse},
    node::OutputsOptions as RustOutputsOptions,
    OutputType,
};

use crate::{
    classes::address::{Address, AddressDto},
    Result,
};

#[derive(Getters, CopyGetters, Clone, Serialize, Deserialize)]
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
    #[getset(get_copy = "pub")]
    pub ledger_index: u32,
}

impl OutputResponse {
    pub fn output(&self) -> OutputDto {
        self.output.clone()
    }

    pub fn to_rust_output(&self) -> RustOutputResponse {
        RustOutputResponse {
            message_id: self.message_id.clone(),
            transaction_id: self.transaction_id.clone(),
            output_index: self.output_index,
            is_spent: self.is_spent,
            output: self.output.to_inner_clone(),
            ledger_index: self.ledger_index,
        }
    }
}

impl Display for OutputResponse {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "(message_id={}, transaction_id={}, output_index={}, is_spent={}, output=({}))",
            self.message_id, self.transaction_id, self.output_index, self.is_spent, self.output
        )
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
            ledger_index: output.ledger_index,
        }
    }
}

#[derive(Debug)]
pub enum OutputKind {
    SignatureLockedSingle = 1,
    SignatureLockedDustAllowance = 2,
    Treasury = 3,
}

pub fn output_kind_to_type(kind: OutputKind) -> OutputType {
    match kind {
        OutputKind::SignatureLockedSingle => OutputType::SignatureLockedSingle,
        OutputKind::SignatureLockedDustAllowance => OutputType::SignatureLockedDustAllowance,
        _ => unimplemented!(),
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct OutputDto {
    output: RustOutputDto,
}

impl OutputDto {
    pub fn kind(&self) -> OutputKind {
        match self.output {
            RustOutputDto::SignatureLockedSingle(_) => OutputKind::SignatureLockedSingle,
            RustOutputDto::SignatureLockedDustAllowance(_) => OutputKind::SignatureLockedDustAllowance,
            RustOutputDto::Treasury(_) => OutputKind::Treasury,
        }
    }

    pub fn as_signature_locked_single_output_dto(&self) -> Result<SignatureLockedSingleOutputDto> {
        SignatureLockedSingleOutputDto::try_from(&self.output)
    }

    pub fn as_signature_locked_dust_allowance_output_dto(&self) -> Result<SignatureLockedDustAllowanceOutputDto> {
        SignatureLockedDustAllowanceOutputDto::try_from(&self.output)
    }

    pub fn as_treasury_output(&self) -> Result<TreasuryOutputDto> {
        TreasuryOutputDto::try_from(&self.output)
    }

    pub fn to_inner_clone(&self) -> RustOutputDto {
        self.output.clone()
    }
}

impl Display for OutputDto {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.output)
    }
}

impl From<&RustOutput> for OutputDto {
    fn from(output: &RustOutput) -> OutputDto {
        let rust_output: RustOutputDto = output.into();
        OutputDto { output: rust_output }
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
    pub address: AddressDto,
    #[getset(get_copy = "pub")]
    pub amount: u64,
}
impl SignatureLockedSingleOutputDto {
    pub fn address(&self) -> AddressDto {
        self.address.clone()
    }
}

impl Display for SignatureLockedSingleOutputDto {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "(amount={}, address={}, kind={})",
            self.amount, self.address, self.kind
        )
    }
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
    pub address: AddressDto,
    #[getset(get_copy = "pub")]
    pub amount: u64,
}

impl SignatureLockedDustAllowanceOutputDto {
    pub fn address(&self) -> AddressDto {
        self.address.clone()
    }
}

impl Display for SignatureLockedDustAllowanceOutputDto {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "(amount={}, address={}, kind={})",
            self.amount, self.address, self.kind
        )
    }
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

impl Display for TreasuryOutputDto {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "(amount={}, kind={})", self.amount, self.kind)
    }
}

#[derive(Default, Clone)]
pub struct OutputsOptions {
    options: RustOutputsOptions,
}

impl OutputsOptions {
    pub fn include_spent(&mut self, include_spent: bool) {
        self.options.include_spent = include_spent;
    }

    pub fn output_type(&mut self, output_type: Option<OutputKind>) {
        self.options.output_type = output_type.map(output_kind_to_type);
    }

    pub fn to_inner(&self) -> RustOutputsOptions {
        self.options.clone()
    }
}

impl Display for OutputsOptions {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({}, {:?})", self.options.include_spent, self.options.output_type)
    }
}

pub struct Output {
    output: RustOutput,
}

impl Output {
    pub fn kind(&self) -> OutputKind {
        match self.output {
            RustOutput::SignatureLockedSingle(_) => OutputKind::SignatureLockedSingle,
            RustOutput::SignatureLockedDustAllowance(_) => OutputKind::SignatureLockedDustAllowance,
            RustOutput::Treasury(_) => OutputKind::Treasury,
        }
    }

    pub fn as_signature_locked_single_output(&self) -> Result<SignatureLockedSingleOutput> {
        SignatureLockedSingleOutput::try_from(self.output.clone())
    }

    pub fn as_signature_locked_dust_allowance_output(&self) -> Result<SignatureLockedDustAllowanceOutput> {
        SignatureLockedDustAllowanceOutput::try_from(self.output.clone())
    }

    pub fn as_treasury_output(&self) -> Result<TreasuryOutput> {
        TreasuryOutput::try_from(self.output.clone())
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.output)
    }
}

impl From<&RustOutput> for Output {
    fn from(output: &RustOutput) -> Output {
        Output { output: output.clone() }
    }
}

/// Describes a deposit to a single address which is unlocked via a signature.
#[derive(Clone, Debug)]
pub struct SignatureLockedSingleOutput(RustSignatureLockedSingleOutput);

impl SignatureLockedSingleOutput {
    pub fn from(address: Address, amount: u64) -> Result<SignatureLockedSingleOutput> {
        match RustSignatureLockedSingleOutput::new(address.to_inner_clone(), amount) {
            Ok(e) => Ok(Self(e)),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }
    pub fn amount(&self) -> u64 {
        self.0.amount()
    }
    pub fn address(&self) -> Address {
        (*self.0.address()).into()
    }

    pub fn to_inner_clone(&self) -> RustSignatureLockedSingleOutput {
        self.0.clone()
    }
}

impl From<RustSignatureLockedSingleOutput> for SignatureLockedSingleOutput {
    fn from(output: RustSignatureLockedSingleOutput) -> Self {
        Self(output)
    }
}

impl TryFrom<RustOutput> for SignatureLockedSingleOutput {
    type Error = Error;
    fn try_from(output: RustOutput) -> Result<Self, Self::Error> {
        match output {
            RustOutput::SignatureLockedSingle(ed) => Ok(Self(ed)),
            _ => unimplemented!(),
        }
    }
}

impl Display for SignatureLockedSingleOutput {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}
/// Output type for deposits that enables an address to receive dust outputs. It can be consumed as an input like a
/// regular SigLockedSingleOutput.
#[derive(Clone, Debug)]
pub struct SignatureLockedDustAllowanceOutput(RustSignatureLockedDustAllowanceOutput);
impl SignatureLockedDustAllowanceOutput {
    pub fn from(address: Address, amount: u64) -> Result<SignatureLockedDustAllowanceOutput> {
        match RustSignatureLockedDustAllowanceOutput::new(address.to_inner_clone(), amount) {
            Ok(e) => Ok(Self(e)),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn amount(&self) -> u64 {
        self.0.amount()
    }

    pub fn address(&self) -> Address {
        (*self.0.address()).into()
    }
    pub fn to_inner_clone(&self) -> RustSignatureLockedDustAllowanceOutput {
        self.0.clone()
    }
}

impl TryFrom<RustOutput> for SignatureLockedDustAllowanceOutput {
    type Error = Error;
    fn try_from(output: RustOutput) -> Result<Self, Self::Error> {
        match output {
            RustOutput::SignatureLockedDustAllowance(ed) => Ok(Self(ed)),
            _ => unimplemented!(),
        }
    }
}

impl Display for SignatureLockedDustAllowanceOutput {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}

#[derive(Clone, Debug)]
pub struct TreasuryOutput(RustTreasuryOutput);

impl TreasuryOutput {
    pub fn from(amount: u64) -> Result<TreasuryOutput> {
        match RustTreasuryOutput::new(amount) {
            Ok(e) => Ok(Self(e)),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn amount(&self) -> u64 {
        self.0.amount()
    }
    pub fn to_inner_clone(&self) -> RustTreasuryOutput {
        self.0.clone()
    }
}

impl TryFrom<RustOutput> for TreasuryOutput {
    type Error = Error;
    fn try_from(output: RustOutput) -> Result<Self, Self::Error> {
        match output {
            RustOutput::Treasury(ed) => Ok(Self(ed)),
            _ => unimplemented!(),
        }
    }
}
impl From<RustTreasuryOutput> for TreasuryOutput {
    fn from(output: RustTreasuryOutput) -> Self {
        Self(output)
    }
}
impl Display for TreasuryOutput {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "({:?})", self.0)
    }
}
