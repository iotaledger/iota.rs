// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{InputSelection, Requirement},
    block::{
        address::Address,
        output::{AliasId, NftId},
        protocol::protocol_parameters,
    },
    Error,
};

use crate::input_selection::{
    build_inputs, build_outputs,
    Build::{Alias, Basic, Nft},
    ALIAS_ID_1, BECH32_ADDRESS, BECH32_ADDRESS_ALIAS, BECH32_ADDRESS_ED25519, BECH32_ADDRESS_NFT,
    BECH32_ADDRESS_REMAINDER, NFT_ID_1,
};

#[test]
fn sdruc_output_not_provided() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519, 1_000_000)),
    )]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    // assert_eq!(selected.inputs, inputs);
    // assert_eq!(selected.outputs, outputs);
}

// inputs: [
// basic{
// unlock_conditions: [
// address: ‘a’,
// storage_deposit_return { address: ‘b’, amount: ‘1_000_000’}
// ]
// }]
// provided outputs: [basic{ amount: 1_000_000, address: ‘a’ }]
// selected outputs: [
// basic{
// amount: 1_000_000,
// unlock_conditions: [address: ‘a’]
// },
// // the output with return amount
// basic{
// amount: 1_000_000,
// unlock_conditions: [address: ‘b’]
// }]
// expected selected: all inputs
// expected remainder: None
