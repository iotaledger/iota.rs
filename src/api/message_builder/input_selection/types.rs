// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Types used during transaction creation
use crate::api::message_builder::input_selection::InputSigningData;

use bee_message::{
    address::Address,
    output::{Output, TokenId},
};
use bee_rest_api::types::responses::OutputResponse;

use primitive_types::U256;

use std::collections::HashMap;

/// Transaction data with selected inputs, input data for signing and outputs, with remainder output if required
#[derive(Debug, Clone)]
pub struct SelectedTransactionData {
    /// Selected inputs with data for signing
    pub inputs: Vec<InputSigningData>,
    /// All outputs for the transaction, including remainder output if required
    pub outputs: Vec<Output>,
    /// Optional remainder output
    pub remainder_output: Option<Output>,
}

/// Required things from the to be created outputs
#[derive(Debug, Clone)]
pub(crate) struct AccumulatedOutputData {
    pub(crate) amount: u64,
    pub(crate) native_tokens: HashMap<TokenId, U256>,
    // unlock address with the output responses of the required input
    pub(crate) utxo_chains: Vec<(Address, OutputResponse)>,
    // unlock_address with the output responses of the required input
    // pub(crate) chains_with_governance_transition: Vec<(Address, OutputResponse)>,
}
