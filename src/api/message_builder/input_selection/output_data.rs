// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_message::{
    address::Address,
    output::{dto::OutputDto, unlock_condition::dto::UnlockConditionDto, NativeTokensBuilder, Output},
};
use bee_rest_api::types::responses::OutputResponse;
use crypto::keys::slip10::Chain;

use crate::{
    api::{
        input_selection::{get_minted_and_melted_native_tokens, types::AccumulatedOutputAmounts},
        message_builder::ClientMessageBuilder,
        search_address,
    },
    constants::HD_WALLET_TYPE,
    secret::types::InputSigningData,
    Result,
};

// Calculate required accumulated amounts from the outputs, considers also minted and melted native tokens
pub(crate) async fn get_accumulated_output_amounts(
    inputs: &[Output],
    outputs: &[Output],
) -> Result<AccumulatedOutputAmounts> {
    // Calculate the total tokens to spend
    let mut required_amount: u64 = 0;
    let mut required_native_tokens = NativeTokensBuilder::new();

    for output in outputs {
        required_amount += output.amount();

        if let Some(output_native_tokens) = output.native_tokens() {
            required_native_tokens.add_native_tokens(output_native_tokens.clone())?;
        }
    }

    // check if a foundry mints or melts native tokens
    let (minted_native_tokens, melted_native_tokens) = get_minted_and_melted_native_tokens(inputs, outputs)?;
    // add melted native tokens as outputs, because we need to have this amount in the inputs
    required_native_tokens.merge(melted_native_tokens)?;

    Ok(AccumulatedOutputAmounts {
        minted_native_tokens,
        amount: required_amount,
        native_tokens: required_native_tokens,
    })
}

/// Get inputs for utxo chains
pub(crate) async fn get_utxo_chains_inputs(
    message_builder: &ClientMessageBuilder<'_>,
    outputs: &[Output],
) -> Result<Vec<InputSigningData>> {
    log::debug!("[get_utxo_chains_inputs]");
    let client = message_builder.client;
    let bech32_hrp = client.get_bech32_hrp().await?;
    let mut utxo_chains: Vec<(Address, OutputResponse)> = Vec::new();
    for output in outputs {
        match output {
            Output::Alias(alias_output) => {
                // if the state_index is [0u8; 20] then there can't be a previous output and it can also not be a
                // governance transition
                if alias_output.alias_id().as_ref() != [0u8; 20] {
                    // Check if the transaction is a governance_transition, by checking if the new index is the same
                    // as the previous index
                    let output_id = client.alias_output_id(*alias_output.alias_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Alias(output) = &output_response.output {
                        if let OutputDto::Alias(alias_output_dto) = &output_response.output {
                            for unlock_condition in &alias_output_dto.unlock_conditions {
                                // A governance transition is identified by an unchanged State Index in next
                                // state.
                                if alias_output.state_index() == output.state_index {
                                    if let UnlockConditionDto::GovernorAddress(governor_unlock_condition_dto) =
                                        unlock_condition
                                    {
                                        let address = Address::try_from(&governor_unlock_condition_dto.address)?;
                                        utxo_chains.push((address, output_response.clone()));
                                    }
                                } else if let UnlockConditionDto::StateControllerAddress(
                                    state_controller_unlock_condition_dto,
                                ) = unlock_condition
                                {
                                    let address = Address::try_from(&state_controller_unlock_condition_dto.address)?;
                                    utxo_chains.push((address, output_response.clone()));
                                }
                            }
                        }
                    }
                }
            }
            Output::Nft(nft_output) => {
                // If the id is [0u8; 20] then this output creates it and we can't have a previous output
                if nft_output.nft_id().as_ref() != [0u8; 20] {
                    let output_id = client.nft_output_id(*nft_output.nft_id()).await?;
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Nft(nft_output_dto) = &output_response.output {
                        for unlock_condition in &nft_output_dto.unlock_conditions {
                            if let UnlockConditionDto::Address(address_unlock_condition_dto) = unlock_condition {
                                let address = Address::try_from(&address_unlock_condition_dto.address)?;
                                utxo_chains.push((address, output_response.clone()));
                            }
                        }
                    }
                }
            }
            Output::Foundry(foundry_output) => {
                // if it's the first foundry output, then we can't have it as input
                if let Ok(output_id) = client.foundry_output_id(foundry_output.id()).await {
                    let output_response = client.get_output(&output_id).await?;
                    if let OutputDto::Foundry(foundry_output) = &output_response.output {
                        for unlock_condition in &foundry_output.unlock_conditions {
                            if let UnlockConditionDto::ImmutableAliasAddress(
                                immutable_alias_address_unlock_condition_dto,
                            ) = unlock_condition
                            {
                                let address = Address::try_from(&immutable_alias_address_unlock_condition_dto.address)?;
                                utxo_chains.push((address, output_response.clone()));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    let mut utxo_chain_inputs = Vec::new();
    for (unlock_address, output_response) in utxo_chains {
        let (address_index, internal) = match message_builder.secret_manager {
            Some(secmngr) => {
                match unlock_address {
                    Address::Ed25519(_) => {
                        search_address(
                            secmngr,
                            &bech32_hrp,
                            message_builder.coin_type,
                            message_builder.account_index,
                            message_builder.input_range.clone(),
                            &unlock_address,
                        )
                        .await?
                    }
                    // Alias and NFT addresses can't be generated from a private key
                    _ => (0, false),
                }
            }
            None => (0, false),
        };

        utxo_chain_inputs.push(InputSigningData {
            output_response: output_response.clone(),
            chain: Some(Chain::from_u32_hardened(vec![
                HD_WALLET_TYPE,
                message_builder.coin_type,
                message_builder.account_index,
                internal as u32,
                address_index,
            ])),
            bech32_address: unlock_address.to_bech32(&bech32_hrp),
        });
    }

    Ok(utxo_chain_inputs)
}
