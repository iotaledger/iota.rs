// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{Burn, InputSelection, Requirement},
    block::{
        output::{AliasId, Output, SimpleTokenScheme},
        protocol::protocol_parameters,
    },
    Error,
};
use primitive_types::U256;

use crate::input_selection::{
    build_alias_output, build_foundry_output, build_input_signing_data_alias_outputs,
    build_input_signing_data_foundry_outputs, build_input_signing_data_most_basic_outputs, ALIAS_ID_1, ALIAS_ID_2,
    BECH32_ADDRESS,
};

#[test]
fn missing_input_alias_for_foundry() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000, None)]);
    let outputs = vec![build_foundry_output(
        alias_id_2,
        1_000_000,
        SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(10)).unwrap(),
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id))) if alias_id == alias_id_2
    ))
}

#[test]
fn existing_input_alias_for_foundry_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_2, BECH32_ADDRESS, 1_251_500, None)]);
    let outputs = vec![build_foundry_output(
        alias_id_2,
        1_000_000,
        SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(10)).unwrap(),
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .select()
        .unwrap();

    // Alias next state + foundry
    assert_eq!(selected.1.len(), 2);
    // Alias state index is increased
    selected.1.iter().for_each(|output| {
        if let Output::Alias(alias_output) = &output {
            // Input alias has index 0, output should have index 1
            assert_eq!(alias_output.state_index(), 1);
        }
    });
}

#[test]
fn minted_native_tokens_in_new_remainder() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_2, BECH32_ADDRESS, 2_251_500, None)]);
    let outputs = vec![build_foundry_output(
        alias_id_2,
        1_000_000,
        SimpleTokenScheme::new(U256::from(10), U256::from(0), U256::from(10)).unwrap(),
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .select()
        .unwrap();

    // Alias next state + foundry + basic output with native tokens
    assert_eq!(selected.1.len(), 3);
    // Alias state index is increased
    selected.1.iter().for_each(|output| {
        if let Output::Alias(alias_output) = &output {
            // Input alias has index 0, output should have index 1
            assert_eq!(alias_output.state_index(), 1);
        }
        if let Output::Basic(basic_output) = &output {
            // Basic output remainder has the minted native tokens
            assert_eq!(basic_output.native_tokens().first().unwrap().amount(), U256::from(10));
        }
    });
}

#[test]
fn melt_native_tokens() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let mut inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, BECH32_ADDRESS, 1_000_000, None)]);
    inputs.extend(build_input_signing_data_foundry_outputs(vec![(
        alias_id_1,
        1_000_000,
        SimpleTokenScheme::new(U256::from(10), U256::from(0), U256::from(10)).unwrap(),
        Some(vec![(
            "0x0811111111111111111111111111111111111111111111111111111111111111110000000000",
            10,
        )]),
    )]));
    let outputs = vec![build_foundry_output(
        alias_id_1,
        1_000_000,
        // Melt 5 native tokens
        SimpleTokenScheme::new(U256::from(10), U256::from(5), U256::from(10)).unwrap(),
        None,
    )];

    let selected = InputSelection::new(inputs.clone(), outputs, protocol_parameters)
        .select()
        .unwrap();

    // Alias next state + foundry + basic output with native tokens
    assert_eq!(selected.1.len(), 3);
    // Alias state index is increased
    selected.1.iter().for_each(|output| {
        if let Output::Alias(alias_output) = &output {
            // Input alias has index 0, output should have index 1
            assert_eq!(alias_output.state_index(), 1);
        }
        if let Output::Basic(basic_output) = &output {
            // Basic output remainder has the remaining native tokens
            assert_eq!(basic_output.native_tokens().first().unwrap().amount(), U256::from(5));
        }
    });
}

#[test]
fn destroy_foundry() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let mut inputs = build_input_signing_data_alias_outputs(vec![(alias_id_2, BECH32_ADDRESS, 50_300, None)]);
    inputs.extend(build_input_signing_data_foundry_outputs(vec![(
        alias_id_2,
        52_800,
        SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
        None,
    )]));
    // Alias output gets the amount from the foundry output added
    let outputs = vec![build_alias_output(
        103_100,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];

    let selected = InputSelection::new(inputs.clone(), outputs, protocol_parameters)
        .burn(Burn::new().add_foundry(inputs[1].output.as_foundry().id()))
        .select()
        .unwrap();

    // Alias next state
    assert_eq!(selected.1.len(), 1);
}
