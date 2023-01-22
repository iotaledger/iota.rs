// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::{Burn, InputSelection},
    block::{output::AliasId, protocol::protocol_parameters},
    Error,
};

use crate::input_selection::{
    build_inputs, build_outputs, is_remainder_or_return,
    Build::{Alias, Basic},
    ALIAS_ID_2, BECH32_ADDRESS_ED25519_0,
};

#[test]
fn no_inputs() {
    let protocol_parameters = protocol_parameters();

    let inputs = Vec::new();
    let outputs = build_outputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None)]);

    let selected = InputSelection::new(inputs, outputs, vec![], protocol_parameters).select();

    assert!(matches!(selected, Err(Error::NoAvailableInputsProvided)));
}

#[test]
fn no_outputs() {
    let protocol_parameters = protocol_parameters();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS_ED25519_0, None, None, None, None)]);
    let outputs = Vec::new();

    let selected = InputSelection::new(inputs, outputs, vec![], protocol_parameters).select();

    assert!(matches!(selected, Err(Error::NoOutputsProvided)));
}

#[test]
fn no_outputs_but_burn() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(
        2_000_000,
        alias_id_2,
        BECH32_ADDRESS_ED25519_0,
        None,
        None,
        None,
    )]);
    let outputs = Vec::new();

    let selected = InputSelection::new(inputs.clone(), outputs, vec![], protocol_parameters)
        .burn(Burn::new().add_alias(alias_id_2))
        .select()
        .unwrap();

    assert_eq!(selected.inputs, inputs);
    assert_eq!(selected.outputs.len(), 1);
    assert!(is_remainder_or_return(
        &selected.outputs[0],
        2_000_000,
        BECH32_ADDRESS_ED25519_0,
        None
    ));
}
