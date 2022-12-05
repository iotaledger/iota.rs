// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{
    requirement::{alias::is_alias_with_id, foundry::is_foundry_with_id, nft::is_nft_with_id},
    Burn,
};
use crate::{
    block::{
        output::{
            AliasOutput, AliasOutputBuilder, FoundryOutput, FoundryOutputBuilder, NftOutput, NftOutputBuilder, Output,
            OutputId,
        },
        protocol::ProtocolParameters,
    },
    error::Result,
    secret::types::InputSigningData,
};

fn transition_alias_input(
    input: &AliasOutput,
    output_id: &OutputId,
    outputs: &[Output],
    burn: Option<&Burn>,
    protocol_parameters: &ProtocolParameters,
) -> Result<Option<Output>> {
    let alias_id = input.alias_id_non_null(output_id);

    // Don't create an alias output if the alias input is to be burned.
    if burn.map(|burn| burn.aliases.contains(&alias_id)).unwrap_or(false) {
        return Ok(None);
    }

    // Don't create an alias output if it already exists.
    if outputs
        .iter()
        .any(|output| is_alias_with_id(output, output_id, &alias_id))
    {
        return Ok(None);
    }

    // TODO remove sender feature ?
    // TODO minimum amount ?

    let output = AliasOutputBuilder::from(input)
        .with_alias_id(alias_id)
        .with_state_index(input.state_index() + 1)
        .finish_output(protocol_parameters.token_supply())?;

    Ok(Some(output))
}

fn transition_nft_input(
    input: &NftOutput,
    output_id: &OutputId,
    outputs: &[Output],
    burn: Option<&Burn>,
    protocol_parameters: &ProtocolParameters,
) -> Result<Option<Output>> {
    let nft_id = input.nft_id_non_null(output_id);

    // Don't create an nft output if the nft input is to be burned.
    if burn.map(|burn| burn.nfts.contains(&nft_id)).unwrap_or(false) {
        return Ok(None);
    }

    // Don't create an nft output if it already exists.
    if outputs.iter().any(|output| is_nft_with_id(output, output_id, &nft_id)) {
        return Ok(None);
    }

    // TODO remove sender feature ?
    // TODO minimum amount ?

    let output = NftOutputBuilder::from(input)
        .with_nft_id(nft_id)
        .finish_output(protocol_parameters.token_supply())?;

    Ok(Some(output))
}

fn transition_foundry_input(
    input: &FoundryOutput,
    outputs: &[Output],
    burn: Option<&Burn>,
    protocol_parameters: &ProtocolParameters,
) -> Result<Option<Output>> {
    let foundry_id = input.id();

    // Don't create a foundry output if the foundry input is to be burned.
    if burn.map(|burn| burn.foundries.contains(&input.id())).unwrap_or(false) {
        return Ok(None);
    }

    // Don't create a foundry output if it already exists.
    if outputs.iter().any(|output| is_foundry_with_id(output, &foundry_id)) {
        return Ok(None);
    }

    // TODO minimum amount ?

    let output = FoundryOutputBuilder::from(input).finish_output(protocol_parameters.token_supply())?;

    Ok(Some(output))
}

pub(crate) fn transition_input(
    input: &InputSigningData,
    outputs: &[Output],
    burn: Option<&Burn>,
    protocol_parameters: &ProtocolParameters,
) -> Result<Option<Output>> {
    match &input.output {
        Output::Alias(alias_input) => {
            transition_alias_input(alias_input, input.output_id(), outputs, burn, protocol_parameters)
        }
        Output::Nft(nft_input) => {
            transition_nft_input(nft_input, input.output_id(), outputs, burn, protocol_parameters)
        }
        Output::Foundry(foundry_input) => transition_foundry_input(foundry_input, outputs, burn, protocol_parameters),
        _ => Ok(None),
    }
}
