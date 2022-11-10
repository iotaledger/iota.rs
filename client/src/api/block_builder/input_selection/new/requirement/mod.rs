// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod alias;
mod base_token;
mod foundry;
mod issuer;
mod native_tokens;
mod nft;
mod remainder;
mod sender;

use std::collections::VecDeque;

use alias::fulfill_alias_requirement;
use base_token::fulfill_base_coin_requirement;
use foundry::fulfill_foundry_requirement;
use issuer::fulfill_issuer_requirement;
use native_tokens::fulfill_native_tokens_requirement;
use nft::fulfill_nft_requirement;
use remainder::fulfill_remainder_requirement;
use sender::fulfill_sender_requirement;

use super::Burn;
use crate::{
    block::{
        address::Address,
        output::{AliasId, FoundryId, NftId, Output},
    },
    error::Result,
    secret::types::InputSigningData,
};

#[derive(Debug, serde::Serialize)]
pub enum Requirement {
    Sender(Address),
    Issuer(Address),
    Foundry(FoundryId),
    Alias(AliasId),
    Nft(NftId),
    NativeTokens,
    BaseToken,
    Remainder,
}

impl Requirement {
    pub(crate) fn fulfill(
        self,
        available_inputs: &mut Vec<InputSigningData>,
        selected_inputs: &[InputSigningData],
        outputs: &[Output],
    ) -> Result<Vec<InputSigningData>> {
        match self {
            Requirement::Sender(address) => {
                fulfill_sender_requirement(address, available_inputs, selected_inputs, outputs)
            }
            Requirement::Issuer(address) => {
                fulfill_issuer_requirement(address, available_inputs, selected_inputs, outputs)
            }
            Requirement::Foundry(foundry_id) => {
                fulfill_foundry_requirement(foundry_id, available_inputs, selected_inputs, outputs)
            }
            Requirement::Alias(alias_id) => {
                fulfill_alias_requirement(alias_id, available_inputs, selected_inputs, outputs)
            }
            Requirement::Nft(nft_id) => fulfill_nft_requirement(nft_id, available_inputs, selected_inputs, outputs),
            Requirement::NativeTokens => fulfill_native_tokens_requirement(available_inputs, selected_inputs, outputs),
            Requirement::BaseToken => fulfill_base_coin_requirement(available_inputs, selected_inputs, outputs),
            Requirement::Remainder => fulfill_remainder_requirement(available_inputs, selected_inputs, outputs),
        }
    }
}

pub(crate) struct Requirements(VecDeque<Requirement>);

impl Requirements {
    pub(crate) fn new() -> Self {
        Self(VecDeque::new())
    }

    pub(crate) fn push(&mut self, requirement: Requirement) {
        self.0.push_front(requirement)
    }

    pub(crate) fn pop(&mut self) -> Option<Requirement> {
        self.0.pop_front()
    }

    pub(crate) fn extend(&mut self, mut requirements: Requirements) {
        while let Some(requirement) = requirements.pop() {
            self.push(requirement);
        }
    }

    pub(crate) fn from_outputs<'a>(
        inputs: impl Iterator<Item = &'a InputSigningData> + Clone,
        outputs: impl Iterator<Item = &'a Output>,
    ) -> Self {
        // TODO take duplicate into account
        let mut requirements = Requirements::new();

        for output in outputs {
            let is_new: bool = match output {
                // Add an alias requirement if the alias output is transitioning, thus required in the inputs.
                Output::Alias(alias_output) => {
                    let is_new = alias_output.alias_id().is_null();

                    if !is_new {
                        requirements.push(Requirement::Alias(*alias_output.alias_id()));
                    }

                    !is_new
                }
                // Add a nft requirement if the nft output is transitioning, thus required in the inputs.
                Output::Nft(nft_output) => {
                    let is_new = nft_output.nft_id().is_null();

                    if !is_new {
                        requirements.push(Requirement::Nft(*nft_output.nft_id()));
                    }

                    !is_new
                }
                // Add a foundry requirement if the foundry output is transitioning, thus required in the inputs.
                // Also add an alias requirement since the associated alias output needs to be transitioned.
                Output::Foundry(foundry_output) => {
                    let is_new = !inputs.clone().any(|input| {
                        if let Output::Foundry(output) = &input.output {
                            output.id() == foundry_output.id()
                        } else {
                            false
                        }
                    });

                    if !is_new {
                        requirements.push(Requirement::Foundry(foundry_output.id()));
                        // TODO take care of minted and melted tokens somewhere
                    }

                    requirements.push(Requirement::Alias(*foundry_output.alias_address().alias_id()));

                    !is_new
                }
                _ => false,
            };

            if let Some(features) = output.features() {
                // Add a sender requirement if the feature is present.
                if let Some(sender) = features.sender() {
                    requirements.push(Requirement::Sender(*sender.address()));
                }

                // Add an issuer requirement if the feature is present and new.
                if let Some(issuer) = features.issuer() {
                    if is_new {
                        requirements.push(Requirement::Issuer(*issuer.address()));
                    }
                }
            }
        }

        requirements
    }

    pub(crate) fn from_burn(burn: &Burn) -> Self {
        let mut requirements = Requirements::new();

        for alias_id in &burn.aliases {
            requirements.push(Requirement::Alias(*alias_id));
        }

        for nft_id in &burn.nfts {
            requirements.push(Requirement::Nft(*nft_id));
        }

        for foundry_id in &burn.foundries {
            requirements.push(Requirement::Foundry(*foundry_id));
        }

        // TODO add native tokens

        requirements
    }
}
