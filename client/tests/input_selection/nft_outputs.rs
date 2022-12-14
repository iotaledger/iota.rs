// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{Burn, InputSelection, Requirement},
    block::{
        output::{NftId, Output},
        protocol::protocol_parameters,
    },
    Error,
};

use crate::input_selection::{
    build_basic_output, build_input_signing_data_most_basic_outputs, build_input_signing_data_nft_outputs,
    build_nft_output,
};

const BECH32_ADDRESS: &str = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";
const NFT_ID_0: &str = "0x0000000000000000000000000000000000000000000000000000000000000000";
const NFT_ID_1: &str = "0x1111111111111111111111111111111111111111111111111111111111111111";

#[test]
fn input_nft_eq_output_nft() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(nft_id_1, BECH32_ADDRESS, 1_000_000)];

    let selected = InputSelection::build(outputs, inputs.clone(), protocol_parameters)
        .finish()
        .select()
        .unwrap();

    assert_eq!(selected.0, inputs);
}

#[test]
fn input_amount_lt_output_amount() {
    let protocol_parameters = protocol_parameters();
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None)];

    assert!(matches!(
        InputSelection::build(outputs, inputs, protocol_parameters)
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
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, BECH32_ADDRESS, 2_229_500)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(outputs, inputs, protocol_parameters)
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
    let outputs = vec![build_nft_output(nft_id_0, BECH32_ADDRESS, 1_000_000)];

    let selected = InputSelection::build(outputs, inputs, protocol_parameters)
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
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, BECH32_ADDRESS, 2_000_000)]);
    let outputs = vec![build_basic_output(2_000_000, BECH32_ADDRESS, None)];

    let selected = InputSelection::build(outputs, inputs, protocol_parameters)
        .burn(Burn::new().add_nft(nft_id_1))
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
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, BECH32_ADDRESS, 1_000_001)]);
    let outputs = vec![build_nft_output(nft_id_1, BECH32_ADDRESS, 1_000_000)];

    assert!(matches!(
        InputSelection::build(outputs, inputs, protocol_parameters)
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
    let nft_id_1 = NftId::from_str(NFT_ID_1).unwrap();

    let inputs = build_input_signing_data_most_basic_outputs(vec![(BECH32_ADDRESS, 1_000_000)]);
    let outputs = vec![build_nft_output(nft_id_1, BECH32_ADDRESS, 1_000_000)];

    assert!(matches!(
        InputSelection::build(outputs, inputs, protocol_parameters)
            .finish()
            .select(),
        Err(Error::UnfulfillableRequirement(Requirement::Nft(nft_id))) if nft_id == nft_id_1
    ))
}
