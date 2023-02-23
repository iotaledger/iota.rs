// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::{Error, InputSelection},
    block::{output::AliasId, protocol::protocol_parameters},
};

use crate::{
    addresses, build_inputs, build_outputs, is_remainder_or_return, unsorted_eq,
    Build::{Alias, Basic},
    ALIAS_ID_1, BECH32_ADDRESS_ALIAS_1, BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1, BECH32_ADDRESS_ED25519_2,
};

#[test]
fn sdruc_output_not_provided_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        None,
        None,
        None,
    )]);
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

    assert!(unsorted_eq(&selected.inputs, &inputs));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_1,
                None
            ));
        }
    });
}

#[test]
fn sdruc_output_provided_no_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![
        Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None, None, None),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_1, None, None, None, None, None, None),
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
    assert!(unsorted_eq(&selected.outputs, &outputs));
}

#[test]
fn sdruc_output_provided_remainder() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_1,
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_0,
                None
            ));
        }
    });
}

#[test]
fn two_sdrucs_to_the_same_address_both_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
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
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                2_000_000,
                BECH32_ADDRESS_ED25519_1,
                None
            ));
        }
    });
}

#[test]
fn two_sdrucs_to_the_same_address_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            None,
            None,
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
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_1,
                None
            ));
        }
    });
}

#[test]
fn two_sdrucs_to_different_addresses_both_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            None,
            None,
            None,
        ),
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 1_000_000)),
            None,
            None,
            None,
        ),
    ]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
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
    assert!(
        selected
            .outputs
            .iter()
            .any(|output| { is_remainder_or_return(output, 1_000_000, BECH32_ADDRESS_ED25519_1, None) })
    );
    assert!(
        selected
            .outputs
            .iter()
            .any(|output| { is_remainder_or_return(output, 1_000_000, BECH32_ADDRESS_ED25519_2, None) })
    );
}

#[test]
fn two_sdrucs_to_different_addresses_one_needed() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
            None,
            None,
            None,
        ),
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 1_000_000)),
            None,
            None,
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
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_1,
                None
            ));
        }
    });
}

#[test]
fn insufficient_amount_because_of_sdruc() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        Some((BECH32_ADDRESS_ED25519_1, 1_000_000)),
        None,
        None,
        None,
    )]);
    let outputs = build_outputs(vec![Basic(
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
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
            found: 2_000_000,
            required: 3_000_000,
        })
    ));
}

#[test]
fn useless_sdruc_required_for_sender_feature() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ED25519_0,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 1_000_000)),
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ED25519_1, None, None, None, None, None, None),
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_0,
        None,
        Some(BECH32_ADDRESS_ED25519_0),
        None,
        None,
        None,
        None,
    )]);

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![BECH32_ADDRESS_ED25519_0, BECH32_ADDRESS_ED25519_1]),
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
                BECH32_ADDRESS_ED25519_2,
                None
            ));
        }
    });
}

#[test]
fn sdruc_required_non_ed25519_in_address_unlock() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(
            2_000_000,
            BECH32_ADDRESS_ALIAS_1,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 1_000_000)),
            None,
            None,
            None,
        ),
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
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_2,
        None,
        Some(BECH32_ADDRESS_ALIAS_1),
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
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) && !output.is_alias() {
            assert!(is_remainder_or_return(
                output,
                1_000_000,
                BECH32_ADDRESS_ED25519_2,
                None
            ));
        }
    });
}

#[test]
fn useless_sdruc_non_ed25519_in_address_unlock() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Basic(
            1_000_000,
            BECH32_ADDRESS_ALIAS_1,
            None,
            None,
            Some((BECH32_ADDRESS_ED25519_2, 1_000_000)),
            None,
            None,
            None,
        ),
        Basic(1_000_000, BECH32_ADDRESS_ALIAS_1, None, None, None, None, None, None),
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
    ]);
    let outputs = build_outputs(vec![Basic(
        1_000_000,
        BECH32_ADDRESS_ED25519_2,
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

    assert_eq!(selected.inputs.len(), 2);
    assert!(selected.inputs.contains(&inputs[1]));
    assert!(selected.inputs.contains(&inputs[2]));
    assert_eq!(selected.outputs.len(), 2);
    assert!(selected.outputs.contains(&outputs[0]));
    selected.outputs.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_alias());
        }
    });
}
