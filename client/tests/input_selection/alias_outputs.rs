// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{Burn, InputSelection, Requirement},
    block::{
        output::{AliasId, Output},
        protocol::protocol_parameters,
    },
    Error,
};

use crate::input_selection::{
    build_alias_output, build_input_signing_data_alias_outputs, build_input_signing_data_most_basic_outputs,
    build_most_basic_output,
};

const BECH32_ADDRESS: &str = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";
const ALIAS_ID_0: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const ALIAS_ID_1: &str = "0x1111111111111111111111111111111111111111111111111111111111111111";

#[test]
fn input_alias_eq_output_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_alias_output(alias_id_1, BECH32_ADDRESS, 1_000_000)];

    let selected = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
}

#[test]
fn input_amount_lt_output_amount() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_most_basic_output(BECH32_ADDRESS, 2_000_000)];

    assert!(matches!(
        InputSelection::build(outputs, inputs, protocol_parameters.clone())
            .finish()
            .select(),
        Err(Error::NotEnoughBalance {
            found: 1_000_000,
            // Amount we want to send + storage deposit for alias remainder
            required: 2_251_500,
        })
    ))
}

#[test]
fn basic_output_with_alias_input() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, BECH32_ADDRESS, 2_251_500)]);
    let outputs = vec![build_most_basic_output(BECH32_ADDRESS, 2_000_000)];

    let selected = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()
        .unwrap();

    // basic output + alias remainder
    assert_eq!(selected.1.len(), 2);
}

#[test]
fn create_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_alias_output(alias_id_0, BECH32_ADDRESS, 1_000_000)];

    let selected = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()
        .unwrap();

    // One output should be added for the remainder
    assert_eq!(selected.1.len(), 2);
    // Output contains the new minted alias id
    assert!(selected.1.iter().any(|output| {
        if let Output::Alias(alias_output) = output {
            *alias_output.alias_id() == alias_id_0
        } else {
            false
        }
    }));
}

#[test]
fn burn_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_most_basic_output(BECH32_ADDRESS, 2_000_000)];

    let selected = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .burn(Burn::new().add_alias(alias_id_1))
        .finish()
        .select()
        .unwrap();

    // No remainder
    assert_eq!(selected.1.len(), 1);
    // Output is a basic output
    assert!(matches!(selected.1[0], Output::Basic(_)));
}

#[test]
fn not_enough_storage_deposit_for_remainder() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, BECH32_ADDRESS, 1_000_001)]);
    let outputs = vec![build_alias_output(alias_id_1, BECH32_ADDRESS, 1_000_000)];

    assert!(matches!(
        InputSelection::build(outputs, inputs, protocol_parameters.clone())
            .finish()
            .select(),
        Err(Error::BlockError(
            iota_types::block::Error::InsufficientStorageDepositAmount {
                amount: 1,
                required: 213000,
            }
        ))
    ))
}

#[test]
fn missing_input_for_alias_output() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_alias_output(alias_id_1, BECH32_ADDRESS, 1_000_000)];

    assert!(matches!(
        InputSelection::build(outputs, inputs, protocol_parameters.clone())
            .finish()
            .select(),
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id))) if alias_id == alias_id_1
    ))
}
