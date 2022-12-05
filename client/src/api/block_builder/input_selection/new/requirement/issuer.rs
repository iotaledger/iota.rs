// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::fulfill_sender_requirement;
use crate::{block::address::Address, error::Result, secret::types::InputSigningData};

/// Fulfills an issuer requirement by fulfilling the equivalent sender requirement.
/// This is kept in case fulfilling sender and issuer requirements diverges at some point.
pub(crate) fn fulfill_issuer_requirement(
    address: Address,
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
) -> Result<Vec<InputSigningData>> {
    fulfill_sender_requirement(address, available_inputs, selected_inputs)
}
