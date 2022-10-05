// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::input_selection::try_select_inputs, block::output::RentStructure, Error, Result};

use crate::input_selection::{build_input_signing_data_most_basic_outputs, build_most_basic_output};

const TOKEN_SUPPLY: u64 = 1_813_620_509_061_365;

#[test]
fn input_selection_basic_outputs() -> Result<()> {
    let rent_structure = RentStructure::build()
        .byte_cost(500)
        .key_factor(10)
        .data_factor(1)
        .finish();
    let bech32_address = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";

    // input amount == output amount
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 1_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 1_000_000)];
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
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 1_000_000)]);
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
            required: 2_000_000,
        }) => {}
        _ => panic!("Should return NotEnoughBalance"),
    }

    // output amount < input amount
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 2_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 1_000_000)];
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
    // One output should be added for the remainder
    assert_eq!(selected_transaction_data.outputs.len(), 2);

    // 2 inputs, only one needed
    let inputs =
        build_input_signing_data_most_basic_outputs(vec![(bech32_address, 2_000_000), (bech32_address, 2_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 1_000_000)];
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
    // One input has enough amount
    assert_eq!(selected_transaction_data.inputs.len(), 1);
    // One output should be added for the remainder
    assert_eq!(selected_transaction_data.outputs.len(), 2);

    // not enough storage deposit for remainder
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 1_000_001)]);
    let outputs = vec![build_most_basic_output(bech32_address, 1_000_000)];
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

    Ok(())
}
