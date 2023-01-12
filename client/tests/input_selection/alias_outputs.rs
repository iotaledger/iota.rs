// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{Burn, InputSelection, Requirement},
    block::{
        address::Address,
        output::{AliasId, Output},
        protocol::protocol_parameters,
    },
    Error,
};

use crate::input_selection::{
    build_alias_output, build_basic_output, build_inputs, unsorted_eq,
    Build::{Alias, Basic},
    ALIAS_ID_0, ALIAS_ID_1, ALIAS_ID_2, BECH32_ADDRESS, BECH32_ADDRESS_ALIAS_SENDER, BECH32_ADDRESS_ED25519_SENDER,
    BECH32_ADDRESS_NFT_SENDER,
};

#[test]
fn input_alias_eq_output_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_000_000, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
    assert_eq!(selected.1, outputs);
}

#[test]
fn input_amount_lt_output_amount() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_000_000, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientBaseTokenAmount {
            found: 1_000_000,
            // Amount we want to send + storage deposit for alias remainder
            required: 2_251_500,
        })
    ));
}

#[test]
fn input_amount_lt_output_amount_2() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![
        Alias(2_000_000, alias_id_2, BECH32_ADDRESS, None),
        Basic(1_000_000, BECH32_ADDRESS, None),
    ]);
    let outputs = vec![build_basic_output(3_000_001, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::InsufficientBaseTokenAmount {
            found: 3_000_000,
            // Amount we want to send + storage deposit for alias remainder
            required: 3_251_501
        })
    ));
}

#[test]
fn basic_output_with_alias_input() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(2_251_500, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .select()
        .unwrap();

    // basic output + alias remainder
    assert_eq!(selected.1.len(), 2);
}

#[test]
fn create_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(2_000_000, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_0,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
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
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(2_000_000, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters)
        .burn(Burn::new().add_alias(alias_id_2))
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
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_000_001, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];

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
fn missing_input_for_alias_output() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id))) if alias_id == alias_id_2
    ));
}

#[test]
fn missing_input_for_alias_output_2() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![
        Alias(2_000_000, alias_id_1, BECH32_ADDRESS, None),
        Basic(1_000_000, BECH32_ADDRESS, None),
    ]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Alias(alias_id))) if alias_id == alias_id_2
    ));
}

#[test]
fn missing_input_for_alias_output_but_created() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_0,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(selected.is_ok());
}

#[test]
fn alias_in_output_and_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(1_000_000, alias_id_1, BECH32_ADDRESS, None),
        Basic(1_000_000, BECH32_ADDRESS, None),
    ]);
    let mut outputs = vec![build_alias_output(
        1_000_000,
        alias_id_1,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];
    outputs.push(build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ALIAS_SENDER),
    ));

    let selected = InputSelection::new(inputs.clone(), outputs, protocol_parameters)
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.0, &inputs));
    assert_eq!(selected.1.len(), 2);
    assert!(selected.1.iter().any(|output| {
        if let Output::Alias(alias_output) = output {
            *alias_output.alias_id() == alias_id_1
        } else {
            false
        }
    }));
    assert!(selected.1.iter().any(|output| output.is_basic()));
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_000_000, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ED25519_SENDER),
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_ed25519() && sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_SENDER).unwrap().1
    ));
}

#[test]
fn missing_ed25519_issuer_created() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_0,
        BECH32_ADDRESS,
        None,
        None,
        Some(BECH32_ADDRESS_ED25519_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer.is_ed25519() && issuer == Address::try_from_bech32(BECH32_ADDRESS_ED25519_SENDER).unwrap().1
    ));
}

#[test]
fn missing_ed25519_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_000_000, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        None,
        Some(BECH32_ADDRESS_ED25519_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(selected.is_ok());
}

#[test]
fn missing_alias_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_000_000, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ALIAS_SENDER),
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_alias() && sender == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_SENDER).unwrap().1
    ));
}

#[test]
fn missing_alias_issuer_created() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_0,
        BECH32_ADDRESS,
        None,
        None,
        Some(BECH32_ADDRESS_ALIAS_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer.is_alias() && issuer == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_SENDER).unwrap().1
    ));
}

#[test]
fn missing_alias_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_000_000, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        None,
        Some(BECH32_ADDRESS_ALIAS_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(selected.is_ok());
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_000_000, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_NFT_SENDER),
        None,
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_nft() && sender == Address::try_from_bech32(BECH32_ADDRESS_NFT_SENDER).unwrap().1
    ));
}

#[test]
fn missing_nft_issuer_created() {
    let protocol_parameters = protocol_parameters();
    let alias_id_0 = AliasId::from_str(ALIAS_ID_0).unwrap();

    let inputs = build_inputs(vec![Basic(1_000_000, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_0,
        BECH32_ADDRESS,
        None,
        None,
        Some(BECH32_ADDRESS_NFT_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer.is_nft() && issuer == Address::try_from_bech32(BECH32_ADDRESS_NFT_SENDER).unwrap().1
    ));
}

#[test]
fn missing_nft_issuer_transition() {
    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2).unwrap();

    let inputs = build_inputs(vec![Alias(1_000_000, alias_id_2, BECH32_ADDRESS, None)]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_2,
        BECH32_ADDRESS,
        None,
        None,
        Some(BECH32_ADDRESS_NFT_SENDER),
    )];

    let selected = InputSelection::new(inputs, outputs, protocol_parameters).select();

    assert!(selected.is_ok());
}

#[test]
fn increase_alias_amount() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(2_000_000, alias_id_1, BECH32_ADDRESS, None),
        Basic(1_000_000, BECH32_ADDRESS, None),
    ]);
    let outputs = vec![build_alias_output(
        3_000_000,
        alias_id_1,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.0, &inputs));
    assert!(unsorted_eq(&selected.1, &outputs));
}

#[test]
fn decrease_alias_amount() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(2_000_000, alias_id_1, BECH32_ADDRESS, None),
        Basic(1_000_000, BECH32_ADDRESS, None),
    ]);
    let outputs = vec![build_alias_output(
        1_000_000,
        alias_id_1,
        BECH32_ADDRESS,
        None,
        None,
        None,
    )];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0.len(), 1);
    assert_eq!(selected.0[0], inputs[0]);
    assert_eq!(selected.1.len(), 2);
    assert!(selected.1.contains(&outputs[0]));
    selected.1.iter().for_each(|output| {
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
fn prefer_basic_to_alias() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(1_000_000, alias_id_1, BECH32_ADDRESS, None),
        Basic(1_000_000, BECH32_ADDRESS, None),
    ]);
    let outputs = vec![build_basic_output(1_000_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert_eq!(selected.0.len(), 1);
    assert_eq!(selected.0[0], inputs[1]);
    assert_eq!(selected.1, outputs);
}

#[test]
fn take_amount_from_alias_to_fund_basic() {
    let protocol_parameters = protocol_parameters();
    let alias_id_1 = AliasId::from_str(ALIAS_ID_1).unwrap();

    let inputs = build_inputs(vec![
        Alias(2_000_000, alias_id_1, BECH32_ADDRESS, None),
        Basic(1_000_000, BECH32_ADDRESS, None),
    ]);
    let outputs = vec![build_basic_output(1_200_000, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::new(inputs.clone(), outputs.clone(), protocol_parameters)
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.0, &inputs));
    assert_eq!(selected.1.len(), 2);
    assert!(selected.1.contains(&outputs[0]));
    selected.1.iter().for_each(|output| {
        if !outputs.contains(output) {
            assert!(output.is_alias());
            assert_eq!(output.amount(), 1_800_000);
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
