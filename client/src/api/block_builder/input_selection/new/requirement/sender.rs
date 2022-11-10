// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{fulfill_alias_requirement, fulfill_nft_requirement};
use crate::{
    block::{address::Address, output::Output},
    error::{Error, Result},
    secret::types::InputSigningData,
};

pub(crate) fn fulfill_sender_requirement(
    address: Address,
    available_inputs: &mut Vec<InputSigningData>,
    selected_inputs: &[InputSigningData],
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    match address {
        Address::Ed25519(ed25519_address) => {
            todo!()
        }
        Address::Alias(alias_address) => {
            fulfill_alias_requirement(*alias_address.alias_id(), available_inputs, selected_inputs, outputs)
        }
        Address::Nft(nft_address) => {
            fulfill_nft_requirement(*nft_address.nft_id(), available_inputs, selected_inputs, outputs)
        }
    }
}
