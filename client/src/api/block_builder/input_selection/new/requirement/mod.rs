// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

pub(crate) mod alias;
pub(crate) mod base_token;
pub(crate) mod foundry;
pub(crate) mod issuer;
pub(crate) mod native_tokens;
pub(crate) mod nft;
pub(crate) mod sender;

use std::collections::VecDeque;

use super::{Burn, InputSelection, OutputInfo};
use crate::{
    block::{
        address::Address,
        output::{AliasId, FoundryId, NftId, Output},
    },
    error::Result,
    secret::types::InputSigningData,
};

/// A requirement, imposed by outputs, that needs to be resolved by selected inputs.
#[derive(Debug, serde::Serialize, Eq, PartialEq)]
pub enum Requirement {
    /// Sender requirement.
    Sender(Address),
    /// Issuer requirement.
    Issuer(Address),
    /// Foundry requirement.
    Foundry(FoundryId),
    /// Alias requirement.
    Alias(AliasId),
    /// Nft requirement.
    Nft(NftId),
    /// Native tokens requirement.
    NativeTokens,
    /// Base token requirement.
    BaseToken,
}

impl InputSelection {
    /// Fulfills a requirement by selecting the appropriate available inputs.
    /// Returns the selected inputs and an optional new requirement.
    pub(crate) fn fulfill_requirement(
        &mut self,
        requirement: Requirement,
    ) -> Result<(Vec<InputSigningData>, Option<Requirement>)> {
        match requirement {
            Requirement::Sender(address) => self.fulfill_sender_requirement(address),
            Requirement::Issuer(address) => self.fulfill_issuer_requirement(address),
            Requirement::Foundry(foundry_id) => self.fulfill_foundry_requirement(foundry_id),
            Requirement::Alias(alias_id) => self.fulfill_alias_requirement(alias_id),
            Requirement::Nft(nft_id) => self.fulfill_nft_requirement(nft_id),
            Requirement::NativeTokens => self.fulfill_native_tokens_requirement(),
            Requirement::BaseToken => self.fulfill_base_token_requirement(),
        }
    }
}

/// A queue of requirements, imposed by outputs, that need to be resolved by selected inputs.
#[derive(Debug)]
pub(crate) struct Requirements(VecDeque<Requirement>);

impl Requirements {
    /// Creates a new [`Requirements`].
    pub(crate) fn new() -> Self {
        Self(VecDeque::new())
    }

    /// Pushes a new requirement to the queue.
    pub(crate) fn push(&mut self, requirement: Requirement) {
        // TODO push back ?
        self.0.push_front(requirement)
    }

    /// Pops a requirement from the queue.
    pub(crate) fn pop(&mut self) -> Option<Requirement> {
        self.0.pop_front()
    }

    /// Extends the requirement queue with another queue.
    pub(crate) fn extend(&mut self, mut requirements: Requirements) {
        // TODO should this iterate instead ?
        while let Some(requirement) = requirements.pop() {
            self.push(requirement);
        }
    }

    /// Creates a new [`Requirements`] from outputs.
    pub(crate) fn from_outputs<'a>(
        inputs: impl Iterator<Item = &'a InputSigningData> + Clone,
        outputs: impl Iterator<Item = &'a OutputInfo>,
    ) -> Self {
        // TODO take duplicates into account
        let mut requirements = Requirements::new();

        for output in outputs {
            let is_created = match &output.output {
                // Add an alias requirement if the alias output is transitioning and then required in the inputs.
                Output::Alias(alias_output) => {
                    let is_created = alias_output.alias_id().is_null();

                    if !is_created {
                        requirements.push(Requirement::Alias(*alias_output.alias_id()));
                    }

                    !is_created
                }
                // Add an nft requirement if the nft output is transitioning and then required in the inputs.
                Output::Nft(nft_output) => {
                    let is_created = nft_output.nft_id().is_null();

                    if !is_created {
                        requirements.push(Requirement::Nft(*nft_output.nft_id()));
                    }

                    !is_created
                }
                // Add a foundry requirement if the foundry output is transitioning and then required in the inputs.
                // Also add an alias requirement since the associated alias output needs to be transitioned.
                Output::Foundry(foundry_output) => {
                    let is_created = !inputs.clone().any(|input| {
                        if let Output::Foundry(output) = &input.output {
                            output.id() == foundry_output.id()
                        } else {
                            false
                        }
                    });

                    if !is_created {
                        requirements.push(Requirement::Foundry(foundry_output.id()));
                    }

                    requirements.push(Requirement::Alias(*foundry_output.alias_address().alias_id()));

                    !is_created
                }
                _ => false,
            };

            // Add a sender requirement if the sender feature is present.
            if let Some(sender) = output.output.features().and_then(|features| features.sender()) {
                requirements.push(Requirement::Sender(*sender.address()));
            }

            // Add an issuer requirement if the issuer feature is present and the chain output is created.
            if is_created {
                if let Some(issuer) = output
                    .output
                    .immutable_features()
                    .and_then(|features| features.issuer())
                {
                    requirements.push(Requirement::Issuer(*issuer.address()));
                }
            }
        }

        requirements
    }

    /// Creates a new [`Requirements`] from burn.
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

        requirements
    }
}
