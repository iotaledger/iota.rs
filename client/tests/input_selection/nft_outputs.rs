// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{Burn, InputSelection, Requirement},
    block::{
        address::Address,
        output::{NftId, Output},
        protocol::protocol_parameters,
    },
    Error,
};

use crate::input_selection::{
    build_basic_output, build_input_signing_data_most_basic_outputs, build_input_signing_data_nft_outputs,
    build_nft_output, unsorted_eq, BECH32_ADDRESS, BECH32_ADDRESS_ALIAS_SENDER, BECH32_ADDRESS_ED25519_SENDER,
    BECH32_ADDRESS_NFT_SENDER, NFT_ID_0, NFT_ID_1, NFT_ID_2,
};

#[test]
fn input_nft_eq_output_nft() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(1_000_000, nft_id_2, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::build(inputs.clone(), outputs, protocol_parameters)
        .finish()
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
}

#[test]
fn input_amount_lt_output_amount() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None)];

    assert!(matches!(
        InputSelection::build(inputs, outputs, protocol_parameters)
            .finish()
            .select(),
        Err(Error::NotEnoughBalance {
            found: 1_000_000,
            // Amount we want to send + storage deposit for nft remainder
            required: 2_229_500,
        })
    ));
}

#[test]
fn basic_output_with_nft_input() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 2_229_500)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(inputs, outputs, protocol_parameters)
        .finish()
        .select()
        .unwrap();

    // basic output + nft remainder
    assert_eq!(selected.1.len(), 2);
}

#[test]
fn mint_nft() {
    let protocol_parameters = protocol_parameters();
    let nft_id_0 = NftId::from_str(NFT_ID_0).unwrap();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_nft_output(1_000_000, nft_id_0, BECH32_ADDRESS, None, None)];

    let selected = InputSelection::build(inputs, outputs, protocol_parameters)
        .finish()
        .select()
        .unwrap();

    // One output should be added for the remainder
    assert_eq!(selected.1.len(), 2);
    // Output contains the new minted nft id
    assert!(selected.1.iter().any(|output| {
        if let Output::Nft(nft_output) = output {
            *nft_output.nft_id() == nft_id_0
        } else {
            false
        }
    }));
}

#[test]
fn burn_nft() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(inputs, outputs, protocol_parameters)
        .burn(Burn::new().add_nft(nft_id_2))
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
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 1_000_001)]);
    let outputs = vec![build_nft_output(1_000_000, nft_id_2, BECH32_ADDRESS, None, None)];

    assert!(matches!(
        InputSelection::build(inputs, outputs, protocol_parameters)
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
fn missing_input_for_nft_output() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(1_000_000, nft_id_2, BECH32_ADDRESS, None, None)];

    assert!(matches!(
        InputSelection::build(inputs, outputs,protocol_parameters)
            .finish()
            .select(),
        Err(Error::UnfulfillableRequirement(Requirement::Nft(nft_id))) if nft_id == nft_id_2
    ))
}

#[test]
fn nft_in_output_and_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let mut inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, BECH32_ADDRESS, 1_000_000)]);
    inputs.extend(build_input_signing_data_most_basic_outputs(vec![(
        BECH32_ADDRESS,
        1_000_000,
    )]));
    let mut outputs = vec![build_nft_output(1_000_000, nft_id_1, BECH32_ADDRESS, None, None)];
    outputs.push(build_basic_output(
        1_000_000,
        BECH32_ADDRESS,
        Some(BECH32_ADDRESS_NFT_SENDER),
    ));

    let selected = InputSelection::build(inputs.clone(), outputs, protocol_parameters)
        .finish()
        .select()
        .unwrap();

    assert!(unsorted_eq(&selected.0, &inputs));
    assert_eq!(selected.1.len(), 2);
    assert!(selected.1.iter().any(|output| {
        if let Output::Nft(nft_output) = output {
            *nft_output.nft_id() == nft_id_1
        } else {
            false
        }
    }));
    assert!(selected.1.iter().any(|output| output.is_basic()));
}

#[test]
fn missing_ed25519_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS,
        Some(BECH32_ADDRESS_ED25519_SENDER),
        None,
    )];

    let selected = InputSelection::build(inputs, outputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_ed25519() && sender == Address::try_from_bech32(BECH32_ADDRESS_ED25519_SENDER).unwrap().1
    ))
}

#[test]
fn missing_ed25519_issuer() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ED25519_SENDER),
    )];

    let selected = InputSelection::build(inputs, outputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer.is_ed25519() && issuer == Address::try_from_bech32(BECH32_ADDRESS_ED25519_SENDER).unwrap().1
    ))
}

#[test]
fn missing_alias_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS,
        Some(BECH32_ADDRESS_ALIAS_SENDER),
        None,
    )];

    let selected = InputSelection::build(inputs, outputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_alias() && sender == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_SENDER).unwrap().1
    ))
}

#[test]
fn missing_alias_issuer() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_ALIAS_SENDER),
    )];

    let selected = InputSelection::build(inputs, outputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer.is_alias() && issuer == Address::try_from_bech32(BECH32_ADDRESS_ALIAS_SENDER).unwrap().1
    ))
}

#[test]
fn missing_nft_sender() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS,
        Some(BECH32_ADDRESS_NFT_SENDER),
        None,
    )];

    let selected = InputSelection::build(inputs, outputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Sender(sender))) if sender.is_nft() && sender == Address::try_from_bech32(BECH32_ADDRESS_NFT_SENDER).unwrap().1
    ))
}

#[test]
fn missing_nft_issuer() {
    let protocol_parameters = protocol_parameters();
    let nft_id_2 = NftId::from_str(NFT_ID_2).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_2, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(
        1_000_000,
        nft_id_2,
        BECH32_ADDRESS,
        None,
        Some(BECH32_ADDRESS_NFT_SENDER),
    )];

    let selected = InputSelection::build(inputs, outputs, protocol_parameters)
        .finish()
        .select();

    assert!(matches!(
        selected,
        Err(Error::UnfulfillableRequirement(Requirement::Issuer(issuer))) if issuer.is_nft() && issuer == Address::try_from_bech32(BECH32_ADDRESS_NFT_SENDER).unwrap().1
    ))
}
