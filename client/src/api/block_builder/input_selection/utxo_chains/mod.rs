// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! input selection for utxo chains

use std::str::FromStr;

use iota_types::{
    api::response::OutputWithMetadataResponse,
    block::{
        address::Address,
        output::{dto::OutputDto, AliasOutput, NftOutput, Output, OutputId},
        payload::transaction::TransactionId,
    },
};

use crate::{Client, Result};
mod automatic;

/// Get recursively owned alias and nft outputs and add them to the utxo_chains
pub(crate) async fn get_alias_and_nft_outputs_recursively(
    client: &Client,
    utxo_chains: &mut Vec<(Address, OutputWithMetadataResponse)>,
) -> Result<()> {
    log::debug!("[get_alias_and_nft_outputs_recursively]");
    let current_time = client.get_time_checked().await?;
    let token_supply = client.get_token_supply().await?;

    let mut processed_alias_nft_addresses = std::collections::HashSet::new();

    // Add addresses for alias and nft outputs we already have
    for (_unlock_address, output_response) in utxo_chains.iter() {
        let output_id = OutputId::new(
            TransactionId::from_str(&output_response.metadata.transaction_id)?,
            output_response.metadata.output_index,
        )?;

        match Output::try_from_dto(&output_response.output, token_supply)? {
            Output::Alias(alias_output) => {
                processed_alias_nft_addresses.insert(Address::Alias(alias_output.alias_address(&output_id)));
            }
            Output::Nft(nft_output) => {
                processed_alias_nft_addresses.insert(Address::Nft(nft_output.nft_address(&output_id)));
            }
            _ => {}
        }
    }

    let mut processed_utxo_chains = Vec::new();

    // Make the outputs response optional, because we don't know it yet for new required outputs
    let mut utxo_chain_optional_response: Vec<(Address, Option<OutputWithMetadataResponse>)> =
        utxo_chains.iter_mut().map(|(a, o)| (*a, Some(o.clone()))).collect();

    // Get alias or nft addresses when needed or just add the input again
    while let Some((unlock_address, output_response)) = utxo_chain_optional_response.pop() {
        // Don't request outputs for addresses where we already have the output
        if processed_alias_nft_addresses.insert(unlock_address) {
            match unlock_address {
                Address::Alias(address) => {
                    let output_id = client.alias_output_id(*address.alias_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Alias(alias_output_dto) = &output_response.output {
                        let alias_output = AliasOutput::try_from_dto(alias_output_dto, token_supply)?;
                        // State transition if we add them to inputs
                        let alias_unlock_address = alias_output.state_controller_address();
                        // Add address to unprocessed_alias_nft_addresses so we get the required output there
                        // also
                        if alias_unlock_address.is_alias() || alias_unlock_address.is_nft() {
                            utxo_chain_optional_response.push((*alias_unlock_address, None));
                        }
                        processed_utxo_chains.push((*alias_unlock_address, output_response));
                    }
                }
                Address::Nft(address) => {
                    let output_id = client.nft_output_id(*address.nft_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Nft(nft_output) = &output_response.output {
                        let nft_output = NftOutput::try_from_dto(nft_output, token_supply)?;
                        let unlock_address = nft_output
                            .unlock_conditions()
                            .locked_address(nft_output.address(), current_time);
                        // Add address to unprocessed_alias_nft_addresses so we get the required output there also
                        if unlock_address.is_alias() || unlock_address.is_nft() {
                            utxo_chain_optional_response.push((*unlock_address, None));
                        }
                        processed_utxo_chains.push((*unlock_address, output_response));
                    }
                }
                _ => {}
            }
        }

        // Add if the output_response is available
        if let Some(output_response) = output_response {
            processed_utxo_chains.push((unlock_address, output_response));
        }
    }

    *utxo_chains = processed_utxo_chains;

    Ok(())
}
