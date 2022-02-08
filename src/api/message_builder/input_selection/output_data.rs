// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    api::input_selection::{get_remainder_native_tokens, types::AccumulatedOutputData},
    Client, Error, Result,
};

use bee_message::{
    address::Address,
    output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, NativeToken, Output, TokenId,
    },
};
use bee_rest_api::types::{
    dtos::{OutputDto, UnlockConditionDto},
    responses::OutputResponse,
};

use primitive_types::U256;

use std::collections::{hash_map::Entry, HashMap};

// Get a remainder with amount and native tokens if necessary, if no remainder_address is provided it will be selected
// from the inputs
pub(crate) async fn get_remainder(
    client: &Client,
    inputs: &Vec<Output>,
    outputs: &Vec<Output>,
    remainder_address: Option<Address>,
) -> Result<Option<Output>> {
    let mut remainder_output = None;
    let input_data = get_accumulated_output_data(client, inputs).await?;
    let output_data = get_accumulated_output_data(client, outputs).await?;
    if input_data.amount < output_data.amount {
        return Err(Error::NotEnoughBalance(input_data.amount, output_data.amount));
    }
    let remainder_amount = output_data.amount - input_data.amount;

    let native_token_remainder = get_remainder_native_tokens(&input_data.native_tokens, &output_data.native_tokens);

    // Output possible remaining tokens back to the original address
    if remainder_amount > 0 {
        let remainder_addr = match remainder_address {
            Some(address) => address,
            // get address from an input, by default we only allow ed25519 addresses as remainder, because then we're
            // sure that the sender can access it
            None => {
                let mut address = None;
                for input in inputs {
                    if let Output::Basic(basic_output) = input {
                        for unlock_condition in basic_output.unlock_conditions() {
                            if let UnlockCondition::Address(address_unlock_condition) = unlock_condition {
                                address.replace(address_unlock_condition.address());
                                break;
                            }
                        }
                    }
                }
                match address {
                    Some(addr) => *addr,
                    None => return Err(Error::MissingInputWithEd25519UnlockCondition),
                }
            }
        };

        let mut remainder_output_builder = BasicOutputBuilder::new(remainder_amount)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(remainder_addr)));
        if let Some(remainder_native_tokens) = native_token_remainder {
            for (token_id, amount) in remainder_native_tokens {
                remainder_output_builder =
                    remainder_output_builder.add_native_token(NativeToken::new(token_id, amount)?);
            }
        }
        remainder_output.replace(Output::Basic(remainder_output_builder.finish()?));
    } else {
        // if we have remaining native tokens, but no amount left, then we can't create this transaction
        if native_token_remainder.is_some() {
            return Err(Error::NotEnoughBalanceForNativeTokenRemainder);
        }
    }

    Ok(remainder_output)
}

// gets required amounts and for utxo chains also the required inputs
pub(crate) async fn get_accumulated_output_data(
    client: &Client,
    outputs: &Vec<Output>,
) -> Result<AccumulatedOutputData> {
    // Calculate the total tokens to spend
    let mut required_amount: u64 = 0;
    let mut required_native_tokens: HashMap<TokenId, U256> = HashMap::new();
    // let mut chains_with_governance_transition: Vec<(Address, OutputResponse)> = Vec::new();
    let mut utxo_chains: Vec<(Address, OutputResponse)> = Vec::new();
    for output in outputs {
        required_amount += output.amount();
        if let Some(output_native_tokens) = output.native_tokens() {
            for native_token in output_native_tokens {
                match required_native_tokens.entry(*native_token.token_id()) {
                    Entry::Vacant(e) => {
                        e.insert(*native_token.amount());
                    }
                    Entry::Occupied(mut e) => {
                        *e.get_mut() += *native_token.amount();
                    }
                }
            }
        }
        // todo verify if that's the right way to do this
        match output {
            Output::Alias(alias_output) => {
                // if the state_index is [0u8; 20] then there can't be a previous output and it can also not be a
                // governance transition
                if alias_output.alias_id().as_ref() != &[0u8; 20] {
                    // Check if the transaction is a governance_transition, by checking if the new index is the same as
                    // the previous index
                    let output_ids = client.alias_output_ids(*alias_output.alias_id()).await?;
                    let outputs = client.get_outputs(output_ids).await?;
                    for output_response in outputs {
                        if let OutputDto::Alias(output) = &output_response.output {
                            if let OutputDto::Alias(alias_output_dto) = &output_response.output {
                                for unlock_condition in &alias_output_dto.unlock_conditions {
                                    // A governance transition is identified by an unchanged State Index in next state.
                                    if alias_output.state_index() == output.state_index {
                                        if let UnlockConditionDto::GovernorAddress(governor_unlock_condition_dto) =
                                            unlock_condition
                                        {
                                            let address = Address::try_from(&governor_unlock_condition_dto.address)?;
                                            utxo_chains.push((address, output_response.clone()));
                                        }
                                    } else {
                                        if let UnlockConditionDto::StateControllerAddress(
                                            state_controller_unlock_condition_dto,
                                        ) = unlock_condition
                                        {
                                            let address =
                                                Address::try_from(&state_controller_unlock_condition_dto.address)?;
                                            utxo_chains.push((address, output_response.clone()));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Output::Nft(nft_output) => {
                // If the id is [0u8; 20] then this output creates it and we can't have a previous output
                if nft_output.nft_id().as_ref() != &[0u8; 20] {
                    let output_ids = client.nft_output_ids(*nft_output.nft_id()).await?;
                    let outputs = client.get_outputs(output_ids).await?;
                    for output_response in outputs {
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
            }
            Output::Foundry(foundry_output) => {
                let output_ids = client.foundry_output_ids(foundry_output.id()).await?;
                let outputs = client.get_outputs(output_ids).await?;
                for output_response in outputs {
                    if let OutputDto::Foundry(foundry_output) = &output_response.output {
                        for unlock_condition in &foundry_output.unlock_conditions {
                            if let UnlockConditionDto::Address(address_unlock_condition_dto) = unlock_condition {
                                let address = Address::try_from(&address_unlock_condition_dto.address)?;
                                utxo_chains.push((address, output_response.clone()));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
    Ok(AccumulatedOutputData {
        amount: required_amount,
        native_tokens: required_native_tokens,
        utxo_chains,
        // chains_with_governance_transition,
    })
}
