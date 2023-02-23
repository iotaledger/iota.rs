// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::{Burn, Error, InputSelection},
    block::{output::TokenId, protocol::protocol_parameters},
};
use primitive_types::U256;

use crate::{
    addresses, build_inputs, build_outputs, is_remainder_or_return, unsorted_eq, Build::Basic,
    BECH32_ADDRESS_ED25519_0, TOKEN_ID_1, TOKEN_ID_2,
};

#[test]
fn two_native_tokens_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 150)]),
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_0,
                Some(vec![(TOKEN_ID_1, 50), (TOKEN_ID_2, 100)])
            ));
        }
    });
}

#[test]
fn two_native_tokens_both_needed_plus_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 150), (TOKEN_ID_2, 100)]),
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_0,
                Some(vec![(TOKEN_ID_1, 50)])
            ));
        }
    });
}

#[test]
fn three_inputs_two_needed_plus_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 120)]),
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_0,
                Some(vec![(TOKEN_ID_1, 80)])
            ));
        }
    });
}

#[test]
fn three_inputs_two_needed_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 200)]),
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs,
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 2);
    assert_eq!(selected.outputs, outputs);
}

#[test]
fn insufficient_native_tokens_one_input() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 150)]),
        None,
        None,
        None,
        None,
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
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(100) && required == U256::from(150)));
}

#[test]
fn insufficient_native_tokens_three_inputs() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 301)]),
        None,
        None,
        None,
        None,
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
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(300) && required == U256::from(301)));
}

#[test]
fn burn_and_send_at_the_same_time() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50), (TOKEN_ID_2, 100)]),
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
    .burn(Burn::new().add_native_token(TokenId::from_str(TOKEN_ID_1).unwrap(), 10))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_0,
                Some(vec![(TOKEN_ID_1, 40)])
            ));
        }
    });
}

#[test]
fn burn_one_input_no_output() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        Vec::new(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_native_token(TokenId::from_str(TOKEN_ID_1).unwrap(), 50))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 1);
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50)])
    ));
}

#[test]
fn burn_two_inputs_no_output() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);

    let selected = InputSelection::new(
        inputs.clone(),
        Vec::new(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_native_token(TokenId::from_str(TOKEN_ID_1).unwrap(), 50))
    .select()
    .unwrap();

    assert_eq!(selected.inputs.len(), 1);
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 1);
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50)])
    ));
}

#[test]
fn burn_one_input_two_tokens_no_output() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        Vec::new(),
        addresses(vec![BECH32_ADDRESS_ED25519_0]),
        protocol_parameters,
    )
    .burn(Burn::new().add_native_token(TokenId::from_str(TOKEN_ID_1).unwrap(), 50))
    .select()
    .unwrap();

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 1);
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50), (TOKEN_ID_2, 100)])
    ));
}

#[test]
fn multiple_native_tokens_1() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
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
    assert!(selected.inputs.contains(&inputs[0]));
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn multiple_native_tokens_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 150)]),
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50), (TOKEN_ID_2, 100)])
    ));
}

#[test]
fn multiple_native_tokens_3() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100), (TOKEN_ID_2, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 200)]),
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_2, 100)])
    ));
}

#[test]
fn insufficient_native_tokens() {
    let protocol_parameters = protocol_parameters();

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
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 150)]),
        None,
        None,
        None,
        None,
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
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(0) && required == U256::from(150)));
}

#[test]
fn insufficient_native_tokens_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 150)]),
        None,
        None,
        None,
        None,
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
        }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(100) && required == U256::from(150)));
}

#[test]
fn insufficient_amount_for_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50)]),
        None,
        None,
        None,
        None,
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
        Err(Error::InsufficientAmount {
            found: 1_000_000,
            required: 1_248_000,
        })
    ));
}

#[test]
fn single_output_native_token_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
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
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn single_output_native_token_remainder_1() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50)]),
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50)])
    ));
}

#[test]
fn single_output_native_token_remainder_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
        None,
        None,
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        BECH32_ADDRESS_ED25519_0,
        None
    ));
}

#[test]
fn two_basic_outputs_1() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 200)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        500_000,
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
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
    ));
}

#[test]
fn two_basic_outputs_2() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 200)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50)]),
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
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50)]),
    ));
}

#[test]
fn two_basic_outputs_3() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 200)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 75)]),
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
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 25)]),
    ));
}

#[test]
fn two_basic_outputs_4() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 200)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
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
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        BECH32_ADDRESS_ED25519_0,
        None,
    ));
}

#[test]
fn two_basic_outputs_5() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 200)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 100)]),
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
    assert!(selected.inputs.contains(&inputs[0]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        BECH32_ADDRESS_ED25519_0,
        None,
    ));
}

#[test]
fn two_basic_outputs_6() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 200)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 250)]),
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        1_500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 50)]),
    ));
}

#[test]
fn two_basic_outputs_7() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 200)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 300)]),
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        1_500_000,
        BECH32_ADDRESS_ED25519_0,
        None,
    ));
}

#[test]
fn two_basic_outputs_8() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 200)]),
            None,
            None,
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        500_000,
        BECH32_ADDRESS_ED25519_0,
        Some(vec![(TOKEN_ID_1, 350)]),
        None,
        None,
        None,
        None,
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
            }) if token_id == TokenId::from_str(TOKEN_ID_1).unwrap() && found == U256::from(300) && required == U256::from(350)));
}

#[test]
fn two_basic_outputs_native_tokens_not_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            Some(vec![(TOKEN_ID_1, 100)]),
            None,
            None,
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        500_000,
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
    assert!(selected.inputs.contains(&inputs[1]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    assert!(is_remainder_or_return(
        &selected.outputs[1],
        500_000,
        BECH32_ADDRESS_ED25519_0,
        None,
    ));
}

// T27: :wavy_dash:
// inputs: [basic{ amount: 1_000_000, native_tokens: [{‘a’: 100}] }, basic{ amount: 1_000_000, native_tokens: [{‘a’:
// 200}] }] }] outputs: [basic{ amount: 500_000, native_tokens: [{‘a’: 150}] }]
// expected selected: [basic{ amount: 1_000_000, native_tokens: [{‘a’: 200}] }]
// expected remainder: Some(basic{ amount: 500_000, native_tokens: [{‘a’: 50}] })

// T28: :wavy_dash:
// inputs: [basic{ amount: 1_000_000, native_tokens: [{‘a’: 100}] }, basic{ amount: 1_000_000, native_tokens: [{‘a’:
// 200}] }] }] outputs: [basic{ amount: 500_000, native_tokens: [{‘a’: 200}] }]
// expected selected: [basic{ amount: 1_000_000, native_tokens: [{‘a’: 200}] }]
// expected remainder: Some(basic{ amount: 500_000 })
