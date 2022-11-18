// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::Burn;
use crate::{
    block::output::{AliasOutput, FoundryOutput, NftOutput, Output, OutputId},
    secret::types::InputSigningData,
};

fn transition_alias_input(input: &AliasOutput, output_id: &OutputId, burn: Option<&Burn>) -> Option<Output> {
    if burn
        .map(|burn| burn.aliases.contains(&input.alias_id_non_null(output_id)))
        .unwrap_or(false)
    {
        return None;
    }

    None
}

fn transition_nft_input(input: &NftOutput, output_id: &OutputId, burn: Option<&Burn>) -> Option<Output> {
    if burn
        .map(|burn| burn.nfts.contains(&input.nft_id_non_null(output_id)))
        .unwrap_or(false)
    {
        return None;
    }

    None
}

fn transition_foundry_input(input: &FoundryOutput, burn: Option<&Burn>) -> Option<Output> {
    if burn.map(|burn| burn.foundries.contains(&input.id())).unwrap_or(false) {
        return None;
    }

    None
}

pub(crate) fn transition_input(input: &InputSigningData, outputs: &[Output], burn: Option<&Burn>) -> Option<Output> {
    match &input.output {
        Output::Alias(alias_input) => transition_alias_input(alias_input, input.output_id(), burn),
        Output::Nft(nft_input) => transition_nft_input(nft_input, input.output_id(), burn),
        Output::Foundry(foundry_input) => transition_foundry_input(foundry_input, burn),
        _ => None,
    }
}
