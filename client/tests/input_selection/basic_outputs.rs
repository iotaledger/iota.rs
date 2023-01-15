// Copyright 2022 IOTA Stiftung
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
fn input_amount_equal_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.inputs, inputs);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn input_amount_lower_than_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);
    let outputs = build_outputs(vec![Basic(2_000_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientBaseTokenAmount {
            found: 1_000_000,
            required: 2_000_000,
        })
    ));
}

#[test]
fn input_amount_lower_than_output_amount_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(3_500_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientBaseTokenAmount {
            found: 3_000_000,
            required: 3_500_000,
        })
    ));
}

#[test]
fn input_amount_greater_than_output_amount() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(2_000_000, BECH32_ADDRESS, None, None)]);
    let outputs = build_outputs(vec![Basic(500_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.inputs, inputs);
    // One output should be added for the remainder.
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_500_000);
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
fn input_amount_greater_than_output_amount_with_remainder_address() {
    let protocol_parameters = protocol_parameters();
    let remainder_address = Address::try_from_bech32(BECH32_ADDRESS_REMAINDER).unwrap().1;

    let inputs = build_inputs(vec![Basic(2_000_000, BECH32_ADDRESS, None, None)]);
    let outputs = build_outputs(vec![Basic(500_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .remainder_address(remainder_address)
        .select()
        .unwrap();

    assert_eq!(selected.inputs, inputs);
    // One output should be added for the remainder.
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_500_000);
            assert_eq!(output.as_basic().native_tokens().len(), 0);
            assert_eq!(output.as_basic().unlock_conditions().len(), 1);
            assert_eq!(output.as_basic().features().len(), 0);
            assert_eq!(*output.as_basic().address(), remainder_address);
        }
    });
}

#[test]
fn two_same_inputs_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(500_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs, outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    // One input has enough amount.
    assert_eq!(selected.inputs.len(), 1);
    // One output should be added for the remainder.
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 1_500_000);
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
fn two_inputs_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.inputs, vec![inputs[0].clone()]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn two_inputs_one_needed_reversed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Basic(1_000_000, BECH32_ADDRESS, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.inputs, vec![inputs[1].clone()]);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn two_inputs_both_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(3_000_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.inputs, inputs);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn two_inputs_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(1_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(2_500_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.inputs, inputs);
    // One output should be added for the remainder.
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_basic());
            assert_eq!(output.amount(), 500_000);
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
fn not_enough_storage_deposit_for_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(1_000_001, BECH32_ADDRESS, None, None)]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, None)]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::BlockError(
            iota_types::block::Error::InsufficientStorageDepositAmount {
                amount: 1,
                required: 213000,
            }
        ))
    ));
}

#[test]
fn ed25519_sender() {
    let protocol_parameters = protocol_parameters();
    let sender = Address::try_from_bech32(BECH32_ADDRESS_ED25519).unwrap().1;

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ED25519),
    )]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .select()
        .unwrap();

    // Sender + another for amount
    assert_eq!(selected.inputs.len(), 2);
    assert!(
        selected
            .inputs
            .iter()
            .any(|input| *input.output.as_basic().address() == sender)
    );
    // Provided output + remainder
    assert_eq!(selected.outputs.len(), 2);
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(5_000_000, BECH32_ADDRESS, None, None)]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ED25519),
    )]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_ed25519() && sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519).unwrap().1
    ));
}

#[test]
fn alias_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Alias(1_000_000, alias_id_1, BECH32_ADDRESS, None, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
    ]);

    let outputs = build_outputs(vec![Basic(2_000_000, BECH32_ADDRESS, None, Some(BECH32_ADDRESS_ALIAS))]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .select()
        .unwrap();

    // Sender + another for amount
    assert_eq!(selected.inputs.len(), 2);
    assert!(
        selected
            .inputs
            .iter()
            .any(|input| input.output.is_alias() && *input.output.as_alias().alias_id() == alias_id_1)
    );
    // Provided output + alias
    assert_eq!(selected.outputs.len(), 2);
}

#[test]
fn missing_alias_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(5_000_000, BECH32_ADDRESS, None, None)]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, Some(BECH32_ADDRESS_ALIAS))]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_alias() && sender == Address::try_from_bech32(BECH32_ADDRESS_ALIAS).unwrap().1
    ));
}

#[test]
fn nft_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Nft(1_000_000, nft_id_1, BECH32_ADDRESS, None, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
        Basic(2_000_000, BECH32_ADDRESS, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(2_000_000, BECH32_ADDRESS, None, Some(BECH32_ADDRESS_NFT))]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .select()
        .unwrap();

    // Sender + another for amount
    assert_eq!(selected.inputs.len(), 2);
    assert!(
        selected
            .inputs
            .iter()
            .any(|input| input.output.is_nft() && *input.output.as_nft().nft_id() == nft_id_1)
    );
    // Provided output + nft
    assert_eq!(selected.outputs.len(), 2);
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(5_000_000, BECH32_ADDRESS, None, None)]);
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS, None, Some(BECH32_ADDRESS_NFT))]);

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_nft() && sender == Address::try_from_bech32(BECH32_ADDRESS_NFT).unwrap().1
    ));
}
