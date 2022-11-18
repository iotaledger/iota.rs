// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::fulfill_sender_requirement;
use crate::{
    block::{address::Address, output::Output},
    error::Result,
    secret::types::InputSigningData,
};

pub(crate) fn fulfill_issuer_requirement(
    address: Address,
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    fulfill_sender_requirement(address, available_inputs, selected_inputs, outputs)
}