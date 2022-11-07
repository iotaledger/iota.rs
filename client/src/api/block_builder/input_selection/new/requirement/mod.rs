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

use alias::fulfill_alias_requirement;
use base_token::fulfill_base_coin_requirement;
use foundry::fulfill_foundry_requirement;
use issuer::fulfill_issuer_requirement;
use native_tokens::fulfill_native_tokens_requirement;
use nft::fulfill_nft_requirement;
use remainder::fulfill_remainder_requirement;
use sender::fulfill_sender_requirement;

use crate::block::{
    address::Address,
    output::{AliasId, FoundryId, NftId, Output},
};

enum Requirement {
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
    fn fulfill(&self, available_inputs: &[Output], selected_inputs: &[Output], outputs: &[Output]) -> Vec<Output> {
        // TODO check if selected_inputs already solves the requirement
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
