// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{Burn, InputSelection, Requirement},
    block::{
        address::Address,
        output::{AliasId, Output, SimpleTokenScheme},
        protocol::protocol_parameters,
    },
    Error,
};
use primitive_types::U256;

use crate::input_selection::{
    build_alias_output, build_basic_output, build_foundry_output, build_inputs,
    Build::{Alias, Basic, Foundry},
    ALIAS_ID_1, ALIAS_ID_2, BECH32_ADDRESS,
};

#[test]
fn missing_input_alias_for_foundry() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None)]);
    let outputs = vec![build_foundry_output(
        1_000_000,
        alias_id_2,
        SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(10)).unwrap(),
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id))) if alias_id == alias_id_2
    ));
}

#[test]
fn existing_input_alias_for_foundry_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_251_500, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_foundry_output(
        1_000_000,
        alias_id_2,
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

    let inputs = build_inputs(vec![Alias(2_251_500, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_foundry_output(
        1_000_000,
        alias_id_2,
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

    let inputs = build_inputs(vec![
        Alias(1_000_000, alias_id_1, BECH32_ADDRESS, None),
        Foundry(
            1_000_000,
            alias_id_1,
            SimpleTokenScheme::new(U256::from(10), U256::from(0), U256::from(10)).unwrap(),
            Some(vec![(
                "0x0811111111111111111111111111111111111111111111111111111111111111110000000000",
                10,
            )]),
        ),
    ]);
    let outputs = vec![build_foundry_output(
        1_000_000,
        alias_id_1,
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

    let inputs = build_inputs(vec![
        Alias(50_300, alias_id_2, BECH32_ADDRESS, None),
        Foundry(
            52_800,
            alias_id_2,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
    ]);
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

#[test]
fn prefer_basic_to_foundry() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(1_000_000, alias_id_1, BECH32_ADDRESS, None),
        Foundry(
            1_000_000,
            alias_id_1,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS, None),
    ]);
    let outputs = vec![build_basic_output(1_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0.len(), 1);
    assert_eq!(selected.0[0], inputs[2]);
    assert_eq!(selected.1, outputs);
}

#[test]
fn simple_foundry_transition_basic_not_needed() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS, None),
        Foundry(
            1_000_000,
            alias_id_1,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
        Alias(2_000_000, alias_id_1, BECH32_ADDRESS, None),
    ]);
    let outputs = vec![build_foundry_output(
        1_000_000,
        alias_id_1,
        SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
        None,
    )];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0.len(), 2);
    assert!(selected.0.contains(&inputs[1]));
    assert!(selected.0.contains(&inputs[2]));
    assert_eq!(selected.1.len(), 2);
    assert!(selected.1.contains(&outputs[0]));
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_alias());
            assert_eq!(output.amount(), 2_000_000);
            assert_eq!(output.as_alias().native_tokens().len(), 0);
            assert_eq!(*output.as_alias().alias_id(), alias_id_1);
            assert_eq!(output.as_alias().unlock_conditions().len(), 2);
            assert_eq!(output.as_alias().features().len(), 0);
            assert_eq!(output.as_alias().immutable_features().len(), 0);
            assert_eq!(
                *output.as_alias().state_controller_address(),
                Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
            );
            assert_eq!(
                *output.as_alias().governor_address(),
                Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
            );
        }
    });
}

#[test]
fn simple_foundry_transition_basic_not_needed_with_remainder() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS, None),
        Foundry(
            2_000_000,
            alias_id_1,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
        Alias(2_000_000, alias_id_1, BECH32_ADDRESS, None),
    ]);
    let outputs = vec![build_foundry_output(
        1_000_000,
        alias_id_1,
        SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
        None,
    )];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0.len(), 2);
    assert!(selected.0.contains(&inputs[1]));
    assert!(selected.0.contains(&inputs[2]));
    assert_eq!(selected.1.len(), 3);
    assert!(selected.1.contains(&outputs[0]));
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            if output.is_alias() {
                assert_eq!(output.amount(), 2_000_000);
                assert_eq!(output.as_alias().native_tokens().len(), 0);
                assert_eq!(*output.as_alias().alias_id(), alias_id_1);
                assert_eq!(output.as_alias().unlock_conditions().len(), 2);
                assert_eq!(output.as_alias().features().len(), 0);
                assert_eq!(output.as_alias().immutable_features().len(), 0);
                assert_eq!(
                    *output.as_alias().state_controller_address(),
                    Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
                );
                assert_eq!(
                    *output.as_alias().governor_address(),
                    Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
                );
            } else if output.is_basic() {
                assert_eq!(output.amount(), 1_000_000);
                assert_eq!(output.as_basic().native_tokens().len(), 0);
                assert_eq!(output.as_basic().unlock_conditions().len(), 1);
                assert_eq!(output.as_basic().features().len(), 0);
                assert_eq!(
                    *output.as_basic().address(),
                    Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
                );
            } else {
                panic!("unexpected output type")
            }
        }
    });
}

// #[test]
// fn alias_required_through_sender_and_sufficient() {
//     let protocol_parameters = protocol_parameters();
//     let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

//     let mut inputs = build_inputs(vec![(BECH32_ADDRESS, 1_000_000, None)]);
//     inputs.extend(build_input_signing_data_foundry_outputs(vec![(
//         alias_id_1,
//         2_000_000,
//         SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
//         None,
//     )]));
//     inputs.extend(build_inputs(vec![(
//         alias_id_1,
//         BECH32_ADDRESS,
//         2_000_000,
//         None,
//     )]));
//     let outputs = vec![build_basic_output(
//         1_000_000,
//         BECH32_ADDRESS,
//         None,
//         Some(BECH32_ADDRESS_ALIAS_SENDER),
//     )];

//     let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
//         .select()
//         .unwrap();

//     println!("{selected:?}");

//     assert_eq!(selected.0.len(), 1);
//     assert!(selected.0.contains(&inputs[2]));
//     // assert_eq!(selected.1.len(), 3);
//     // assert!(selected.1.contains(&outputs[0]));
//     // selected.1.iter().for_each(|output| {
//     //     if !outputs.contains(output) {
//     //         if output.is_alias() {
//     //             assert_eq!(output.amount(), 2_000_000);
//     //             assert_eq!(output.as_alias().native_tokens().len(), 0);
//     //             assert_eq!(*output.as_alias().alias_id(), alias_id_1);
//     //             assert_eq!(output.as_alias().unlock_conditions().len(), 2);
//     //             assert_eq!(output.as_alias().features().len(), 0);
//     //             assert_eq!(output.as_alias().immutable_features().len(), 0);
//     //             assert_eq!(
//     //                 *output.as_alias().state_controller_address(),
//     //                 Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
//     //             );
//     //             assert_eq!(
//     //                 *output.as_alias().governor_address(),
//     //                 Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
//     //             );
//     //         } else if output.is_basic() {
//     //             assert_eq!(output.amount(), 1_000_000);
//     //             assert_eq!(output.as_basic().native_tokens().len(), 0);
//     //             assert_eq!(output.as_basic().unlock_conditions().len(), 1);
//     //             assert_eq!(output.as_basic().features().len(), 0);
//     //             assert_eq!(
//     //                 *output.as_basic().address(),
//     //                 Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
//     //             );
//     //         } else {
//     //             panic!("unexpected output type")
//     //         }
//     //     }
//     // });
// }
