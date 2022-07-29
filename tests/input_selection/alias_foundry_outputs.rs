// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::try_select_inputs,
    block::output::{AliasId, Output, RentStructure},
    Error, Result,
};

use crate::input_selection::{
    build_alias_output, build_input_signing_data_alias_outputs, build_input_signing_data_most_basic_outputs,
    build_most_basic_output,
};

#[test]
fn input_selection_alias() -> Result<()> {
    let rent_structure = RentStructure::build()
        .byte_cost(500)
        .key_factor(10)
        .data_factor(1)
        .finish();

    let alias_id_0 = AliasId::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let alias_id_1 = AliasId::from_str("0x1111111111111111111111111111111111111111111111111111111111111111").unwrap();
    let bech32_address = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";

    // input alias == output alias
    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, bech32_address, 1_000_000)]);
    let outputs = vec![build_alias_output(alias_id_1, bech32_address, 1_000_000)];
    let selected_transaction_data = try_select_inputs(inputs.clone(), outputs, false, None, &rent_structure, false, 0)?;
    assert_eq!(selected_transaction_data.inputs, inputs);

    // output amount > input amount
    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, bech32_address, 1_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];
    match try_select_inputs(inputs.clone(), outputs, false, None, &rent_structure, false, 0) {
        Err(Error::NotEnoughBalance {
            found: 1_000_000,
            // Amount we want to send + storage deposit for alias remainder
            required: 2_251_500,
        }) => {}
        _ => panic!("Should return NotEnoughBalance"),
    }

    // basic output with alias as input
    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, bech32_address, 2_251_500)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];
    let selected_transaction_data = try_select_inputs(inputs.clone(), outputs, false, None, &rent_structure, false, 0)?;
    // basic output + alias remainder
    assert_eq!(selected_transaction_data.outputs.len(), 2);

    // mint alias
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 2_000_000)]);
    let outputs = vec![build_alias_output(alias_id_0, bech32_address, 1_000_000)];
    let selected_transaction_data = try_select_inputs(inputs.clone(), outputs, false, None, &rent_structure, false, 0)?;
    // One output should be added for the remainder
    assert_eq!(selected_transaction_data.outputs.len(), 2);
    // Output contains the new minted alias id
    assert!(selected_transaction_data.outputs.iter().any(|output| {
        if let Output::Alias(alias_output) = output {
            *alias_output.alias_id() == alias_id_0
        } else {
            false
        }
    }));

    // burn alias
    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, bech32_address, 2_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];
    let selected_transaction_data = try_select_inputs(inputs.clone(), outputs, false, None, &rent_structure, true, 0)?;
    // No remainder
    assert_eq!(selected_transaction_data.outputs.len(), 1);
    // Output is a basic output
    assert!(if let Output::Basic(_) = selected_transaction_data.outputs[0] {
        true
    } else {
        false
    });

    // not enough storage deposit for remainder
    let inputs = build_input_signing_data_alias_outputs(vec![(alias_id_1, bech32_address, 1_000_001)]);
    let outputs = vec![build_alias_output(alias_id_1, bech32_address, 1_000_000)];
    match try_select_inputs(inputs.clone(), outputs, false, None, &rent_structure, false, 0) {
        Err(Error::BlockError(bee_block::Error::InsufficientStorageDepositAmount {
            amount: 1,
            required: 213000,
        })) => {}
        _ => panic!("Should return InsufficientStorageDepositAmount"),
    }

    // missing input for output alias
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 1_000_000)]);
    let outputs = vec![build_alias_output(alias_id_1, bech32_address, 1_000_000)];
    match try_select_inputs(inputs.clone(), outputs, false, None, &rent_structure, false, 0) {
        Err(Error::MissingInput(err_msg)) => {
            assert_eq!(
                &err_msg,
                "Missing alias input for 0x1111111111111111111111111111111111111111111111111111111111111111"
            );
        }
        _ => panic!("Should return missing alias input"),
    }

    Ok(())
}
