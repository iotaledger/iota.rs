// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use super::Burn;
use crate::{block::output::Output, secret::types::InputSigningData};

pub(crate) fn transition_input(input: &InputSigningData, outputs: &[Output], burn: Option<&Burn>) -> Option<Output> {
    // match input {
    //     Output::Alias(alias_output) => {
    //         if burn.aliases.contains(alias_id) {
    //             return None;
    //         }

    //         Some(
    //                     // TODO create output from input
    //                 )
    //     }
    //     Output::Nft(nft_input) => {
    //         if burn.nfts.contains(nft_id) {
    //             return None;
    //         }

    //         Some(
    //                     // TODO create output from input
    //                 )
    //     }
    //     Output::Foundry(foundry_output) => {
    //         if burn.foundries.contains(foundry_id) {
    //             return None;
    //         }

    //         Some(
    //                     // TODO create output from input
    //                 )
    //     }
    //     _ => None,
    // }

    None
}
