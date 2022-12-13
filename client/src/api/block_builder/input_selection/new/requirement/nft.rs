// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{InputSelection, Requirement};
use crate::{
    block::output::{NftId, Output, OutputId},
    error::{Error, Result},
    secret::types::InputSigningData,
};

/// Checks if an output is an nft with output ID that matches the given nft ID.
pub(crate) fn is_nft_with_id(output: &Output, output_id: &OutputId, nft_id: &NftId) -> bool {
    if let Output::Nft(nft) = output {
        &nft.nft_id_non_null(output_id) == nft_id
    } else {
        false
    }
}

impl InputSelection {
    /// Fulfills a nft requirement by selecting the appropriate nft from the available inputs.
    pub(crate) fn fulfill_nft_requirement(
        &mut self,
        nft_id: NftId,
        selected_inputs: &[InputSigningData],
    ) -> Result<(Vec<InputSigningData>, Option<Requirement>)> {
        // Checks if the requirement is already fulfilled.
        if selected_inputs
            .iter()
            .any(|input| is_nft_with_id(&input.output, input.output_id(), &nft_id))
        {
            return Ok((Vec::new(), None));
        }

        // Checks if the requirement can be fulfilled.
        {
            let index = self
                .available_inputs
                .iter()
                .position(|input| is_nft_with_id(&input.output, input.output_id(), &nft_id));

            match index {
                // Removes the output from the available inputs and returns it, swaps to make it O(1).
                Some(index) => Ok((vec![self.available_inputs.swap_remove(index)], None)),
                None => Err(Error::UnfulfillableRequirement(Requirement::Nft(nft_id))),
            }
        }
    }
}