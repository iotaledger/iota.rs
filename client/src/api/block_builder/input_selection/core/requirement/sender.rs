// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::{Error, InputSelection, Requirement};
use crate::{
    block::{address::Address, output::AliasTransition},
    secret::types::InputSigningData,
};

impl InputSelection {
    /// Fulfills a sender requirement by selecting an available input that unlocks its address.
    pub(crate) fn fulfill_sender_requirement(
        &mut self,
        address: Address,
    ) -> Result<Vec<(InputSigningData, Option<AliasTransition>)>, Error> {
        match address {
            Address::Ed25519(_) => {
                log::debug!("Treating {address:?} sender requirement as an ed25519 requirement");

                match self.fulfill_ed25519_requirement(address) {
                    Ok(res) => Ok(res),
                    Err(Error::UnfulfillableRequirement(Requirement::Ed25519(_))) => {
                        Err(Error::UnfulfillableRequirement(Requirement::Sender(address)))
                    }
                    Err(e) => Err(e),
                }
            }
            Address::Alias(alias_address) => {
                log::debug!("Treating {address:?} sender requirement as an alias requirement");

                // A state transition is required to unlock the alias address.
                match self.fulfill_alias_requirement(alias_address.into_alias_id(), AliasTransition::State) {
                    Ok(res) => Ok(res),
                    Err(Error::UnfulfillableRequirement(Requirement::Alias(_, _))) => {
                        Err(Error::UnfulfillableRequirement(Requirement::Sender(address)))
                    }
                    Err(e) => Err(e),
                }
            }
            Address::Nft(nft_address) => {
                log::debug!("Treating {address:?} sender requirement as an nft requirement");

                match self.fulfill_nft_requirement(nft_address.into_nft_id()) {
                    Ok(res) => Ok(res),
                    Err(Error::UnfulfillableRequirement(Requirement::Nft(_))) => {
                        Err(Error::UnfulfillableRequirement(Requirement::Sender(address)))
                    }
                    Err(e) => Err(e),
                }
            }
        }
    }
}
