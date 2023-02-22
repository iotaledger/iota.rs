// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{
    requirement::{alias::is_alias_with_id_non_null, foundry::is_foundry_with_id, nft::is_nft_with_id_non_null},
    Error, InputSelection,
};
use crate::{
    block::output::{
        AliasOutput, AliasOutputBuilder, AliasTransition, ChainId, FoundryOutput, FoundryOutputBuilder, NftOutput,
        NftOutputBuilder, Output, OutputId,
    },
    secret::types::InputSigningData,
};

impl InputSelection {
    /// Transitions an alias input by creating a new alias output if required.
    fn transition_alias_input(
        &mut self,
        input: &AliasOutput,
        output_id: &OutputId,
        alias_transition: AliasTransition,
    ) -> Result<Option<Output>, Error> {
        let alias_id = input.alias_id_non_null(output_id);

        // Do not create an alias output if the alias input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.aliases.contains(&alias_id))
            .unwrap_or(false)
        {
            log::debug!("No transition of {output_id:?}/{alias_id:?} as it needs to be burned");
            return Ok(None);
        }

        // Do not create an alias output if it already exists.
        if self
            .outputs
            .iter()
            .any(|output| is_alias_with_id_non_null(output, &alias_id))
        {
            log::debug!("No transition of {output_id:?}/{alias_id:?} as output already exists");
            return Ok(None);
        }

        // Remove potential sender feature because it will not be needed anymore as it only needs to be verified once.
        let features = input.features().iter().cloned().filter(|feature| !feature.is_sender());

        let mut builder = AliasOutputBuilder::from(input)
            .with_alias_id(alias_id)
            .with_features(features);

        if alias_transition.is_state() {
            builder = builder.with_state_index(input.state_index() + 1)
        };

        let output = builder.finish_output(self.protocol_parameters.token_supply())?;

        self.automatically_transitioned
            .insert(ChainId::from(alias_id), Some(alias_transition));

        log::debug!("Automatic {alias_transition} transition of {output_id:?}/{alias_id:?}");

        Ok(Some(output))
    }

    /// Transitions an nft input by creating a new nft output if required.
    fn transition_nft_input(&mut self, input: &NftOutput, output_id: &OutputId) -> Result<Option<Output>, Error> {
        let nft_id = input.nft_id_non_null(output_id);

        // Do not create an nft output if the nft input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.nfts.contains(&nft_id))
            .unwrap_or(false)
        {
            log::debug!("No transition of {output_id:?}/{nft_id:?} as it needs to be burned");
            return Ok(None);
        }

        // Do not create an nft output if it already exists.
        if self
            .outputs
            .iter()
            .any(|output| is_nft_with_id_non_null(output, &nft_id))
        {
            log::debug!("No transition of {output_id:?}/{nft_id:?} as output already exists");
            return Ok(None);
        }

        // Remove potential sender feature because it will not be needed anymore as it only needs to be verified once.
        let features = input.features().iter().cloned().filter(|feature| !feature.is_sender());

        let output = NftOutputBuilder::from(input)
            .with_nft_id(nft_id)
            .with_features(features)
            .finish_output(self.protocol_parameters.token_supply())?;

        self.automatically_transitioned.insert(ChainId::from(nft_id), None);

        log::debug!("Automatic transition of {output_id:?}/{nft_id:?}");

        Ok(Some(output))
    }

    /// Transitions a foundry input by creating a new foundry output if required.
    fn transition_foundry_input(
        &mut self,
        input: &FoundryOutput,
        output_id: &OutputId,
    ) -> Result<Option<Output>, Error> {
        let foundry_id = input.id();

        // Do not create a foundry output if the foundry input is to be burned.
        if self
            .burn
            .as_ref()
            .map(|burn| burn.foundries.contains(&foundry_id))
            .unwrap_or(false)
        {
            log::debug!("No transition of {output_id:?}/{foundry_id:?} as it needs to be burned");
            return Ok(None);
        }

        // Do not create a foundry output if it already exists.
        if self
            .outputs
            .iter()
            .any(|output| is_foundry_with_id(output, &foundry_id))
        {
            log::debug!("No transition of {output_id:?}/{foundry_id:?} as output already exists");
            return Ok(None);
        }

        let output = FoundryOutputBuilder::from(input).finish_output(self.protocol_parameters.token_supply())?;

        self.automatically_transitioned.insert(ChainId::from(foundry_id), None);

        log::debug!("Automatic transition of {output_id:?}/{foundry_id:?}");

        Ok(Some(output))
    }

    /// Transitions an input by creating a new output if required.
    /// If no `alias_transition` is provided, assumes a state transition.
    pub(crate) fn transition_input(
        &mut self,
        input: &InputSigningData,
        alias_transition: Option<AliasTransition>,
    ) -> Result<Option<Output>, Error> {
        match &input.output {
            Output::Alias(alias_input) => self.transition_alias_input(
                alias_input,
                input.output_id(),
                alias_transition.unwrap_or(AliasTransition::State),
            ),
            Output::Nft(nft_input) => self.transition_nft_input(nft_input, input.output_id()),
            Output::Foundry(foundry_input) => self.transition_foundry_input(foundry_input, input.output_id()),
            _ => Ok(None),
        }
    }
}
