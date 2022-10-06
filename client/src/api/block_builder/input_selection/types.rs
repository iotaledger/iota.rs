// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types used during transaction creation

use iota_types::block::output::{NativeTokensBuilder, Output};

use crate::api::{block_builder::input_selection::InputSigningData, RemainderData};

/// Transaction data with selected inputs, input data for signing and outputs, with remainder output if required
#[derive(Debug, Clone)]
pub struct SelectedTransactionData {
    /// Selected inputs with data for signing
    pub inputs: Vec<InputSigningData>,
    /// All outputs for the transaction, including remainder output if required
    pub outputs: Vec<Output>,
    /// Optional remainder output, also already parts of the outputs
    pub remainder: Option<RemainderData>,
}

/// Required things from the to be created outputs
#[derive(Debug, Clone)]
pub(crate) struct AccumulatedOutputAmounts {
    pub(crate) minted_native_tokens: NativeTokensBuilder,
    pub(crate) amount: u64,
    pub(crate) native_tokens: NativeTokensBuilder,
}
