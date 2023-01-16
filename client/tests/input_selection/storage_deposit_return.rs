// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{
    api::input_selection::new::InputSelection,
    block::{address::Address, protocol::protocol_parameters},
    Error,
};

use crate::input_selection::{
    build_inputs, build_outputs, unsorted_eq, Build::Basic, BECH32_ADDRESS, BECH32_ADDRESS_ED25519,
    BECH32_ADDRESS_ED25519_2,
};

#[test]
fn sdruc_output_not_provided_no_remainder() {
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

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_000_000);
            assert_eq!(output.as_basic().native_tokens().len(), 0);
            assert_eq!(output.as_basic().unlock_conditions().len(), 1);
            assert_eq!(output.as_basic().features().len(), 0);
            assert_eq!(
                *output.as_basic().address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519).unwrap().1
            );
        }
    });
}

#[test]
fn sdruc_output_provided_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519, 1_000_000)),
    )]);
    let outputs = build_outputs(vec![
        Basic(1_000_000, BECH32_ADDRESS, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519, None, None, None),
    ]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn sdruc_output_provided_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519, 1_000_000)),
    )]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_000_000);
            assert_eq!(output.as_basic().native_tokens().len(), 0);
            assert_eq!(output.as_basic().unlock_conditions().len(), 1);
            assert_eq!(output.as_basic().features().len(), 0);
            assert_eq!(
                *output.as_basic().address(),
                Address::try_from_bech32(BECH32_ADDRESS).unwrap().1
            );
        }
    });
}

#[test]
fn two_sdrucs_to_the_same_address_both_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519, 1_000_000)),
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519, 1_000_000)),
        ),
    ]);
    let outputs = build_outputs(vec![Basic(2_000_000, BECH32_ADDRESS, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 2_000_000);
            assert_eq!(output.as_basic().native_tokens().len(), 0);
            assert_eq!(output.as_basic().unlock_conditions().len(), 1);
            assert_eq!(output.as_basic().features().len(), 0);
            assert_eq!(
                *output.as_basic().address(),
                Address::try_from_bech32(BECH32_ADDRESS_ED25519).unwrap().1
            );
        }
    });
}

// #[test]
// fn two_sdrucs_to_the_same_address_one_needed() {
//     let protocol_parameters = protocol_parameters();

//     let inputs = build_inputs(vec![
//         Basic(
//             2_000_000,
//             BECH32_ADDRESS,
//             None,
//             None,
//             Some((BECH32_ADDRESS_ED25519, 1_000_000)),
//         ),
//         Basic(
//             1_000_000,
//             BECH32_ADDRESS,
//             None,
//             None,
//             Some((BECH32_ADDRESS_ED25519, 1_000_000)),
//         ),
//     ]);
//     let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None, None)]);

//     let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
//         .select()
//         .unwrap();

//     assert_eq!(selected.inputs.len(), 1);
//     assert!(selected.inputs.contains(&inputs[0]));
//     // assert_eq!(selected.outputs.len(), 2);
//     // assert!(selected.outputs.contains(&outputs[0]));
//     // selected.outputs.iter().for_each(|output| {
//     //     if !outputs.contains(output) {
//     //         assert!(output.is_basic());
//     //         assert_eq!(output.amount(), 2_000_000);
//     //         assert_eq!(output.as_basic().native_tokens().len(), 0);
//     //         assert_eq!(output.as_basic().unlock_conditions().len(), 1);
//     //         assert_eq!(output.as_basic().features().len(), 0);
//     //         assert_eq!(
//     //             *output.as_basic().address(),
//     //             Address::try_from_bech32(BECH32_ADDRESS_ED25519).unwrap().1
//     //         );
//     //     }
//     // });
// }

#[test]
fn two_sdrucs_to_different_addresses() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519, 1_000_000)),
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 1_000_000)),
        ),
    ]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 3);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            if *output.as_basic().address() == Address::try_from_bech32(BECH32_ADDRESS_ED25519).unwrap().1 {
                assert_eq!(output.amount(), 1_000_000);
                assert_eq!(output.as_basic().native_tokens().len(), 0);
                assert_eq!(output.as_basic().unlock_conditions().len(), 1);
                assert_eq!(output.as_basic().features().len(), 0);
            } else if *output.as_basic().address() == Address::try_from_bech32(BECH32_ADDRESS_ED25519_2).unwrap().1 {
                assert_eq!(output.amount(), 1_000_000);
                assert_eq!(output.as_basic().native_tokens().len(), 0);
                assert_eq!(output.as_basic().unlock_conditions().len(), 1);
                assert_eq!(output.as_basic().features().len(), 0);
            } else {
                panic!("invalid sdruc output");
            }
        }
    });
}

#[test]
fn insufficient_amount_because_of_sdruc() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519, 1_000_000)),
    )]);
    let outputs = build_outputs(vec![Basic(2_000_000, BECH32_ADDRESS, None, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientBaseTokenAmount {
            found: 2_000_000,
            required: 3_000_000,
        })
    ));
}
