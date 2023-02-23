// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection, Requirement};
use crate::{
    block::output::{AliasTransition, NftId, Output, OutputId},
    secret::types::InputSigningData,
};

/// Checks if an output is an nft with a given nft ID.
/// Assumes that the output nft ID can be null and hashes the output ID.
/// Use when not sure if the nft has been assigned a proper ID already.
pub(crate) fn is_nft_with_id(output: &Output, nft_id: &NftId, output_id: &OutputId) -> bool {
    if let Output::Nft(nft) = output {
        &nft.nft_id_non_null(output_id) == nft_id
    } else {
        false
    }
}

/// Checks if an output is an nft with a given nft ID.
/// Assumes that the output nft ID is non null to avoid an output ID hash.
/// Only use when sure that the nft has been assigned a proper ID already.
pub(crate) fn is_nft_with_id_non_null(output: &Output, nft_id: &NftId) -> bool {
    if let Output::Nft(nft) = output {
        nft.nft_id() == nft_id
    } else {
        false
    }
}

impl InputSelection {
    /// Fulfills an nft requirement by selecting the appropriate nft from the available inputs.
    pub(crate) fn fulfill_nft_requirement(
        &mut self,
        nft_id: NftId,
    ) -> Result<Vec<(InputSigningData, Option<AliasTransition>)>, Error> {
        // Check if the requirement is already fulfilled.
        if let Some(input) = self
            .selected_inputs
            .iter()
            .find(|input| is_nft_with_id(&input.output, &nft_id, input.output_id()))
        {
            log::debug!("{nft_id:?} requirement already fulfilled by {:?}", input.output_id());
            return Ok(vec![]);
        }

        // Check if the requirement can be fulfilled.
        let index = self
            .available_inputs
            .iter()
            .position(|input| is_nft_with_id(&input.output, &nft_id, input.output_id()))
            .ok_or(Error::UnfulfillableRequirement(Requirement::Nft(nft_id)))?;
        // Remove the input from the available inputs, swap to make it O(1).
        let input = self.available_inputs.swap_remove(index);

        log::debug!("{nft_id:?} requirement fulfilled by {:?}", input.output_id());

        Ok(vec![(input, None)])
    }
}
