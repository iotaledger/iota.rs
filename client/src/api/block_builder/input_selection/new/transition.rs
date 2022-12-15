// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{
    requirement::{alias::is_alias_with_id, foundry::is_foundry_with_id, nft::is_nft_with_id},
    InputSelection,
};
use crate::{
    block::output::{
        AliasOutput, AliasOutputBuilder, FoundryOutput, FoundryOutputBuilder, NftOutput, NftOutputBuilder, Output,
        OutputId,
    },
    error::Result,
    secret::types::InputSigningData,
};

impl InputSelection {
    fn transition_alias_input(&self, input: &AliasOutput, output_id: &OutputId) -> Result<Option<Output>> {
        let alias_id = input.alias_id_non_null(output_id);

        // Don't create an alias output if the alias input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.aliases.contains(&alias_id))
            .unwrap_or(false)
        {
            return Ok(None);
        }

        // Don't create an alias output if it already exists.
        if self
            .outputs
            .iter()
            .any(|output| is_alias_with_id(&output.output, output_id, &alias_id))
        {
            return Ok(None);
        }

        // TODO remove sender feature ?
        // TODO minimum amount ?

        let output = AliasOutputBuilder::from(input)
            .with_alias_id(alias_id)
            .with_state_index(input.state_index() + 1)
            .finish_output(self.protocol_parameters.token_supply())?;

        Ok(Some(output))
    }

    fn transition_nft_input(&self, input: &NftOutput, output_id: &OutputId) -> Result<Option<Output>> {
        let nft_id = input.nft_id_non_null(output_id);

        // Don't create an nft output if the nft input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.nfts.contains(&nft_id))
            .unwrap_or(false)
        {
            return Ok(None);
        }

        // Don't create an nft output if it already exists.
        if self
            .outputs
            .iter()
            .any(|output| is_nft_with_id(&output.output, output_id, &nft_id))
        {
            return Ok(None);
        }

        // TODO remove sender feature ?
        // TODO minimum amount ?

        let output = NftOutputBuilder::from(input)
            .with_nft_id(nft_id)
            .finish_output(self.protocol_parameters.token_supply())?;

        Ok(Some(output))
    }

    fn transition_foundry_input(&self, input: &FoundryOutput) -> Result<Option<Output>> {
        let foundry_id = input.id();

        // Don't create a foundry output if the foundry input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.foundries.contains(&input.id()))
            .unwrap_or(false)
        {
            return Ok(None);
        }

        // Don't create a foundry output if it already exists.
        if self
            .outputs
            .iter()
            .any(|output| is_foundry_with_id(&output.output, &foundry_id))
        {
            return Ok(None);
        }

        // TODO minimum amount ?

        let output = FoundryOutputBuilder::from(input).finish_output(self.protocol_parameters.token_supply())?;

        Ok(Some(output))
    }

    pub(crate) fn transition_input(&self, input: &InputSigningData) -> Result<Option<Output>> {
        match &input.output {
            Output::Alias(alias_input) => self.transition_alias_input(alias_input, input.output_id()),
            Output::Nft(nft_input) => self.transition_nft_input(nft_input, input.output_id()),
            Output::Foundry(foundry_input) => self.transition_foundry_input(foundry_input),
            _ => Ok(None),
        }
    }
}
