// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::try_select_inputs,
    block::output::{NftId, Output, RentStructure},
    Error, Result,
};

use crate::input_selection::{
    build_input_signing_data_most_basic_outputs, build_input_signing_data_nft_outputs, build_most_basic_output,
    build_nft_output,
};

const TOKEN_SUPPLY: u64 = 1_813_620_509_061_365;

#[test]
fn input_selection_nfts() -> Result<()> {
    let rent_structure = RentStructure::build()
        .byte_cost(500)
        .key_factor(10)
        .data_factor(1)
        .finish();

    let nft_id_0 = NftId::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let nft_id_1 = NftId::from_str("0x1111111111111111111111111111111111111111111111111111111111111111").unwrap();
    let bech32_address = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";

    // input nft == output nft
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 1_000_000)]);
    let outputs = vec![build_nft_output(nft_id_1, bech32_address, 1_000_000)];
    let selected_transaction_data = try_select_inputs(
        Vec::new(),
        inputs.clone(),
        outputs,
        None,
        &rent_structure,
        false,
        0,
        TOKEN_SUPPLY,
    )?;
    assert_eq!(selected_transaction_data.inputs, inputs);

    // output amount > input amount
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 1_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];
    match try_select_inputs(
        Vec::new(),
        inputs,
        outputs,
        None,
        &rent_structure,
        false,
        0,
        TOKEN_SUPPLY,
    ) {
        Err(Error::NotEnoughBalance {
            found: 1_000_000,
            // Amount we want to send + storage deposit for nft remainder
            required: 2_229_500,
        }) => {}
        _ => panic!("Should return NotEnoughBalance"),
    }

    // basic output with nft as input
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 2_229_500)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];
    let selected_transaction_data = try_select_inputs(
        Vec::new(),
        inputs,
        outputs,
        None,
        &rent_structure,
        false,
        0,
        TOKEN_SUPPLY,
    )?;
    // basic output + nft remainder
    assert_eq!(selected_transaction_data.outputs.len(), 2);

    // mint nft
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 2_000_000)]);
    let outputs = vec![build_nft_output(nft_id_0, bech32_address, 1_000_000)];
    let selected_transaction_data = try_select_inputs(
        Vec::new(),
        inputs,
        outputs,
        None,
        &rent_structure,
        false,
        0,
        TOKEN_SUPPLY,
    )?;
    // One output should be added for the remainder
    assert_eq!(selected_transaction_data.outputs.len(), 2);
    // Output contains the new minted nft id
    assert!(selected_transaction_data.outputs.iter().any(|output| {
        if let Output::Nft(nft_output) = output {
            *nft_output.nft_id() == nft_id_0
        } else {
            false
        }
    }));

    // burn nft
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 2_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];
    let selected_transaction_data = try_select_inputs(
        Vec::new(),
        inputs,
        outputs,
        None,
        &rent_structure,
        true,
        0,
        TOKEN_SUPPLY,
    )?;
    // No remainder
    assert_eq!(selected_transaction_data.outputs.len(), 1);
    // Output is a basic output
    assert!(matches!(selected_transaction_data.outputs[0], Output::Basic(_)));

    // not enough storage deposit for remainder
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 1_000_001)]);
    let outputs = vec![build_nft_output(nft_id_1, bech32_address, 1_000_000)];
    match try_select_inputs(
        Vec::new(),
        inputs,
        outputs,
        None,
        &rent_structure,
        false,
        0,
        TOKEN_SUPPLY,
    ) {
        Err(Error::BlockError(bee_block::Error::InsufficientStorageDepositAmount {
            amount: 1,
            required: 213000,
        })) => {}
        _ => panic!("Should return InsufficientStorageDepositAmount"),
    }

    // missing input for output nft
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 1_000_000)]);
    let outputs = vec![build_nft_output(nft_id_1, bech32_address, 1_000_000)];
    match try_select_inputs(
        Vec::new(),
        inputs,
        outputs,
        None,
        &rent_structure,
        false,
        0,
        TOKEN_SUPPLY,
    ) {
        Err(Error::MissingInput(err_msg)) => {
            assert_eq!(
                &err_msg,
                "missing nft input for 0x1111111111111111111111111111111111111111111111111111111111111111"
            );
        }
        _ => panic!("Should return missing nft input"),
    }

    Ok(())
}
