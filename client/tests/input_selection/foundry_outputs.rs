// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashSet, str::FromStr};

use iota_client::{
    api::input_selection::{Burn, Error, InputSelection, Requirement},
    block::{
        address::{Address, AliasAddress},
        output::{AliasId, AliasOutputBuilder, AliasTransition, FoundryId, Output, SimpleTokenScheme, TokenId},
        protocol::protocol_parameters,
    },
};
use primitive_types::U256;

use crate::{
    addresses, build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Alias, Basic, Foundry},
    ALIAS_ID_1, ALIAS_ID_2, BECH32_ADDRESS_ED25519_0, TOKEN_SUPPLY,
};

#[test]
fn missing_input_alias_for_foundry() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_2,
        0,
        SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(10)).unwrap(),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id, AliasTransition::State))) if alias_id == alias_id_2
    ));
}

#[test]
fn existing_input_alias_for_foundry_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        1_251_500,
        alias_id_2,
        0,
        BECH32_ADDRESS_ED25519_0,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_2,
        0,
        SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(10)).unwrap(),
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // Alias next state + foundry
    assert_eq!(selected.outputs.len(), 2);
    // Alias state index is increased
    selected.outputs.iter().for_each(|output| {
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

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Alias(
            1_000_000,
            alias_id_2,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_2,
        0,
        SimpleTokenScheme::new(U256::from(10), U256::from(0), U256::from(10)).unwrap(),
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // Alias next state + foundry + basic output with native tokens
    assert_eq!(selected.outputs.len(), 3);
    // Alias state index is increased
    selected.outputs.iter().for_each(|output| {
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
fn minted_native_tokens_in_provided_output() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();
    let foundry_id = FoundryId::build(&AliasAddress::from(alias_id_2), 0, SimpleTokenScheme::KIND);
    let token_id = TokenId::from(foundry_id);

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Alias(
            1_000_000,
            alias_id_2,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![
        Foundry(
            1_000_000,
            alias_id_2,
            0,
            SimpleTokenScheme::new(U256::from(100), U256::from(0), U256::from(100)).unwrap(),
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(&token_id.to_string(), 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(selected.outputs.contains(&outputs[1]));
    assert!(selected.outputs.iter().any(|output| output.is_alias()));
}

#[test]
fn melt_native_tokens() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            1_000_000,
            alias_id_1,
            0,
            SimpleTokenScheme::new(U256::from(10), U256::from(0), U256::from(10)).unwrap(),
            Some(vec![(
                "0x0811111111111111111111111111111111111111111111111111111111111111110000000000",
                10,
            )]),
        ),
    ]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_1,
        0,
        // Melt 5 native tokens
        SimpleTokenScheme::new(U256::from(10), U256::from(5), U256::from(10)).unwrap(),
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // Alias next state + foundry + basic output with native tokens
    assert_eq!(selected.outputs.len(), 3);
    // Alias state index is increased
    selected.outputs.iter().for_each(|output| {
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
fn destroy_foundry_with_alias_state_transition() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            50_300,
            alias_id_2,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            52_800,
            alias_id_2,
            0,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
    ]);
    let alias_output = AliasOutputBuilder::from(inputs[0].output.as_alias())
        .with_amount(103_100)
        .unwrap()
        .with_state_index(inputs[0].output.as_alias().state_index() + 1)
        .finish_output(TOKEN_SUPPLY)
        .unwrap();
    // Alias output gets the amount from the foundry output added
    let outputs = vec![alias_output];

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_foundry(inputs[1].output.as_foundry().id()))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    // Alias next state
    assert_eq!(selected.outputs.len(), 1);
}

#[test]
fn destroy_foundry_with_alias_governance_transition() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            1_000_000,
            alias_id_2,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            1_000_000,
            alias_id_2,
            0,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
    ]);
    let outputs = vec![inputs[0].output.clone()];

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_foundry(inputs[1].output.as_foundry().id()))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id, AliasTransition::State))) if alias_id == alias_id_2
    ));
}

#[test]
fn destroy_foundry_with_alias_burn() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            1_000_000,
            alias_id_2,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            1_000_000,
            alias_id_2,
            0,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(
        Burn::new()
            .add_foundry(inputs[1].output.as_foundry().id())
            .add_alias(alias_id_2),
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id, AliasTransition::State))) if alias_id == alias_id_2
    ));
}

#[test]
fn prefer_basic_to_foundry() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            1_000_000,
            alias_id_1,
            0,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert_eq!(selected.inputs[0], inputs[2]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn simple_foundry_transition_basic_not_needed() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Foundry(
            1_000_000,
            alias_id_1,
            0,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_1,
        0,
        SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert!(selected.inputs.contains(&inputs[1]));
    assert!(selected.inputs.contains(&inputs[2]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
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
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap().1
            );
            assert_eq!(
                *output.as_alias().governor_address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap().1
            );
        }
    });
}

#[test]
fn simple_foundry_transition_basic_not_needed_with_remainder() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Foundry(
            2_000_000,
            alias_id_1,
            0,
            SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
            None,
        ),
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_1,
        0,
        SimpleTokenScheme::new(U256::from(10), U256::from(10), U256::from(10)).unwrap(),
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert!(selected.inputs.contains(&inputs[1]));
    assert!(selected.inputs.contains(&inputs[2]));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
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
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap().1
                );
                assert_eq!(
                    *output.as_alias().governor_address(),
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap().1
                );
            } else if output.is_basic() {
                assert!(is_remainder_or_return(
                    output,
                    1_000_000,
                    BECH32_ADDRESS_ED25519_0,
                    None,
                ));
            } else {
                panic!("unexpected output type")
            }
        }
    });
}

// TODO
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
//     let outputs = build_outputs(vec![Basic(
//         1_000_000,
//         BECH32_ADDRESS,
//         None,
//         Some(BECH32_ADDRESS_ALIAS_SENDER),
//     )];

//     let selected = InputSelection::new(inputs.clone(), outputs.clone(), vec![],protocol_parameters)
//         .select()
//         .unwrap();

//     assert_eq!(selected.inputs.len(), 1);
//     assert!(selected.inputs.contains(&inputs[2]));
//     // assert_eq!(selected.outputs.len(), 3);
//     // assert!(selected.outputs.contains(&outputs[0]));
//     // selected.outputs.iter().for_each(|output| {
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

#[test]
fn mint_and_burn_at_the_same_time() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();
    let foundry_id = FoundryId::build(&AliasAddress::from(alias_id_1), 0, SimpleTokenScheme::KIND);
    let token_id = TokenId::from(foundry_id);

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            1_000_000,
            alias_id_1,
            0,
            SimpleTokenScheme::new(U256::from(100), U256::from(0), U256::from(200)).unwrap(),
            Some(vec![(&token_id.to_string(), 100)]),
        ),
    ]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_1,
        0,
        SimpleTokenScheme::new(U256::from(120), U256::from(0), U256::from(200)).unwrap(),
        Some(vec![(&token_id.to_string(), 110)]),
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_native_token(token_id, 10))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            if output.is_alias() {
                assert_eq!(output.amount(), 2_000_000);
                assert_eq!(output.as_alias().native_tokens().len(), 0);
                assert_eq!(output.as_alias().state_index(), 1);
                assert_eq!(*output.as_alias().alias_id(), alias_id_1);
                assert_eq!(output.as_alias().unlock_conditions().len(), 2);
                assert_eq!(output.as_alias().features().len(), 0);
                assert_eq!(output.as_alias().immutable_features().len(), 0);
                assert_eq!(
                    *output.as_alias().state_controller_address(),
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap().1
                );
                assert_eq!(
                    *output.as_alias().governor_address(),
                    Address::try_from_bech32(BECH32_ADDRESS_ED25519_0).unwrap().1
                );
            } else {
                panic!("unexpected output type")
            }
        }
    });
}

#[test]
fn take_amount_from_alias_and_foundry_to_fund_basic() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();
    let foundry_id = FoundryId::build(&AliasAddress::from(alias_id_1), 0, SimpleTokenScheme::KIND);
    let token_id = TokenId::from(foundry_id);

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Foundry(
            1_000_000,
            alias_id_1,
            0,
            SimpleTokenScheme::new(U256::from(100), U256::from(0), U256::from(200)).unwrap(),
            Some(vec![(&token_id.to_string(), 100)]),
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        3_200_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(selected.outputs.iter().any(|output| output.is_alias()));
    assert!(selected.outputs.iter().any(|output| output.is_foundry()));
    assert_eq!(
        selected.outputs.iter().map(|output| output.amount()).sum::<u64>(),
        4_000_000
    );
}

#[test]
fn mint_native_tokens_but_burn_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();
    let foundry_id = FoundryId::build(&AliasAddress::from(alias_id_1), 0, SimpleTokenScheme::KIND);
    let token_id = TokenId::from(foundry_id);

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            1_000_000,
            alias_id_1,
            0,
            SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(100)).unwrap(),
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_1,
        0,
        SimpleTokenScheme::new(U256::from(100), U256::from(0), U256::from(100)).unwrap(),
        Some(vec![(&token_id.to_string(), 100)]),
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_alias(alias_id_1))
    .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id, AliasTransition::State))) if alias_id == alias_id_1
    ));
}

#[test]
fn melted_tokens_not_provided() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();
    let foundry_id = FoundryId::build(&AliasAddress::from(alias_id_1), 0, SimpleTokenScheme::KIND);
    let token_id_1 = TokenId::from(foundry_id);

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            1_000_000,
            alias_id_1,
            0,
            SimpleTokenScheme::new(U256::from(100), U256::from(0), U256::from(100)).unwrap(),
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_1,
        0,
        SimpleTokenScheme::new(U256::from(100), U256::from(100), U256::from(100)).unwrap(),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientNativeTokenAmount {
        token_id,
            found,
            required,
        }) if token_id == token_id_1 && found == U256::from(0) && required == U256::from(100)));
}

#[test]
fn burned_tokens_not_provided() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();
    let foundry_id = FoundryId::build(&AliasAddress::from(alias_id_1), 0, SimpleTokenScheme::KIND);
    let token_id_1 = TokenId::from(foundry_id);

    let inputs = build_inputs(vec![
        Alias(
            2_000_000,
            alias_id_1,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            1_000_000,
            alias_id_1,
            0,
            SimpleTokenScheme::new(U256::from(100), U256::from(0), U256::from(100)).unwrap(),
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_1,
        0,
        SimpleTokenScheme::new(U256::from(100), U256::from(0), U256::from(100)).unwrap(),
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs,
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_native_token(token_id_1, 100))
    .select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientNativeTokenAmount {
        token_id,
            found,
            required,
        }) if token_id == token_id_1 && found == U256::from(0) && required == U256::from(100)));
}

#[test]
fn foundry_in_outputs_and_required() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![
        Alias(
            1_251_500,
            alias_id_2,
            0,
            BECH32_ADDRESS_ED25519_0,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            None,
            None,
        ),
        Foundry(
            1_000_000,
            alias_id_2,
            0,
            SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(10)).unwrap(),
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Foundry(
        1_000_000,
        alias_id_2,
        0,
        SimpleTokenScheme::new(U256::from(0), U256::from(0), U256::from(10)).unwrap(),
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .required_inputs(HashSet::from_iter([*inputs[1].output_id()]))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert_eq!(*output.as_alias().alias_id(), alias_id_2);
        }
    });
}
