// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! input selection for utxo chains

use iota_types::block::output::Output;

use crate::secret::types::InputSigningData;

// Check that for each utxo chain output, the required input is present in the selected inputs
pub(crate) fn check_utxo_chain_inputs(
    selected_inputs: &[InputSigningData],
    outputs: &Vec<Output>,
) -> crate::Result<()> {
    for output in outputs {
        match output {
            Output::Alias(alias_output) => {
                // New created output requires no specific input
                let alias_id = alias_output.alias_id();
                if alias_id.is_null() {
                    continue;
                }

                if !selected_inputs.iter().any(|data| {
                    if let Output::Alias(input_alias_output) = &data.output {
                        input_alias_output.alias_id_non_null(data.output_id()) == *alias_id
                    } else {
                        false
                    }
                }) {
                    return Err(crate::Error::MissingInput(format!(
                        "missing alias input for {alias_id}"
                    )));
                }
            }
            Output::Foundry(foundry_output) => {
                let required_alias = foundry_output.alias_address().alias_id();
                if !selected_inputs.iter().any(|data| {
                    if let Output::Alias(input_alias_output) = &data.output {
                        input_alias_output.alias_id_non_null(data.output_id()) == *required_alias
                    } else {
                        false
                    }
                }) {
                    return Err(crate::Error::MissingInput(format!(
                        "missing alias input {required_alias} for foundry {}",
                        foundry_output.id()
                    )));
                }
            }
            Output::Nft(nft_output) => {
                // New created output requires no specific input
                let nft_id = nft_output.nft_id();
                if nft_id.is_null() {
                    continue;
                }

                if !selected_inputs.iter().any(|data| {
                    if let Output::Nft(input_nft_output) = &data.output {
                        input_nft_output.nft_id_non_null(data.output_id()) == *nft_id
                    } else {
                        false
                    }
                }) {
                    return Err(crate::Error::MissingInput(format!("missing nft input for {nft_id}")));
                }
            }
            _ => {}
        }
    }
    Ok(())
}
