// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::input_selection::new::InputSelection, block::protocol::protocol_parameters, Error, Result};

use crate::input_selection::{build_input_signing_data_most_basic_outputs, build_most_basic_output};

#[test]
fn input_selection_basic_outputs() -> Result<()> {
    let protocol_parameters = protocol_parameters();
    let bech32_address = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";

    // Input amount == output amount.
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 1_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 1_000_000)];
    let selected_transaction_data = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()?;

    assert_eq!(selected_transaction_data.0, inputs);

    // Output amount > input amount.
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 1_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];

    match InputSelection::build(outputs, inputs, protocol_parameters.clone())
        .finish()
        .select()
    {
        Err(Error::NotEnoughBalance {
            found: 1_000_000,
            required: 2_000_000,
        }) => {}
        _ => panic!("Should return NotEnoughBalance"),
    };

    // Output amount < input amount.
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 2_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 1_000_000)];
    let selected_transaction_data = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()?;

    assert_eq!(selected_transaction_data.0, inputs);
    // One output should be added for the remainder.
    assert_eq!(selected_transaction_data.1.len(), 2);

    // Two inputs, only one needed.
    let inputs =
        build_input_signing_data_most_basic_outputs(vec![(bech32_address, 2_000_000), (bech32_address, 2_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 1_000_000)];
    let selected_transaction_data = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()?;

    // One input has enough amount.
    assert_eq!(selected_transaction_data.0.len(), 1);
    // One output should be added for the remainder.
    assert_eq!(selected_transaction_data.1.len(), 2);

    // Not enough storage deposit for remainder.
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 1_000_001)]);
    let outputs = vec![build_most_basic_output(bech32_address, 1_000_000)];

    match InputSelection::build(outputs, inputs.clone(), protocol_parameters)
        .finish()
        .select()
    {
        Err(Error::BlockError(iota_types::block::Error::InsufficientStorageDepositAmount {
            amount: 1,
            required: 213000,
        })) => {}
        _ => panic!("Should return InsufficientStorageDepositAmount"),
    }

    Ok(())
}
