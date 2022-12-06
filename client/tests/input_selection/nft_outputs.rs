// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_client::{
    api::input_selection::new::{Burn, InputSelection, Requirement},
    block::{
        output::{NftId, Output},
        protocol::protocol_parameters,
    },
    Error, Result,
};

use crate::input_selection::{
    build_input_signing_data_most_basic_outputs, build_input_signing_data_nft_outputs, build_most_basic_output,
    build_nft_output,
};

#[test]
fn input_selection_nfts() -> Result<()> {
    let protocol_parameters = protocol_parameters();

    let nft_id_0 = NftId::from_str("0x0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    let nft_id_1 = NftId::from_str("0x1111111111111111111111111111111111111111111111111111111111111111").unwrap();
    let bech32_address = "rms1qr2xsmt3v3eyp2ja80wd2sq8xx0fslefmxguf7tshzezzr5qsctzc2f5dg6";

    // input nft == output nft
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 1_000_000)]);
    let outputs = vec![build_nft_output(nft_id_1, bech32_address, 1_000_000)];
    let selected_transaction_data = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()?;

    assert_eq!(selected_transaction_data.0, inputs);

    println!("START");

    // output amount > input amount
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 1_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];

    match InputSelection::build(outputs, inputs, protocol_parameters.clone())
        .finish()
        .select()
    {
        Err(Error::NotEnoughBalance {
            found: 1_000_000,
            // Amount we want to send + storage deposit for nft remainder
            required: 2_229_500,
        }) => {}
        e => panic!("Should return NotEnoughBalance {e:?}"),
    }

    println!("TEST1");

    // basic output with nft as input
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 2_229_500)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];
    let selected_transaction_data = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()?;

    // basic output + nft remainder
    assert_eq!(selected_transaction_data.1.len(), 2);

    println!("TEST1.1");

    // mint nft
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 2_000_000)]);
    let outputs = vec![build_nft_output(nft_id_0, bech32_address, 1_000_000)];
    let selected_transaction_data = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .finish()
        .select()?;

    // One output should be added for the remainder
    assert_eq!(selected_transaction_data.1.len(), 2);
    // Output contains the new minted nft id
    assert!(selected_transaction_data.1.iter().any(|output| {
        if let Output::Nft(nft_output) = output {
            *nft_output.nft_id() == nft_id_0
        } else {
            false
        }
    }));

    println!("TEST2");

    // burn nft
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 2_000_000)]);
    let outputs = vec![build_most_basic_output(bech32_address, 2_000_000)];
    let selected_transaction_data = InputSelection::build(outputs, inputs.clone(), protocol_parameters.clone())
        .burn(Burn::new().add_nft(nft_id_1))
        .finish()
        .select()?;

    // No remainder
    assert_eq!(selected_transaction_data.1.len(), 1);
    // Output is a basic output
    assert!(matches!(selected_transaction_data.1[0], Output::Basic(_)));

    // not enough storage deposit for remainder
    let inputs = build_input_signing_data_nft_outputs(vec![(nft_id_1, bech32_address, 1_000_001)]);
    let outputs = vec![build_nft_output(nft_id_1, bech32_address, 1_000_000)];

    match InputSelection::build(outputs, inputs, protocol_parameters.clone())
        .finish()
        .select()
    {
        Err(Error::BlockError(iota_types::block::Error::InsufficientStorageDepositAmount {
            amount: 1,
            required: 213000,
        })) => {}
        _ => panic!("Should return InsufficientStorageDepositAmount"),
    }

    println!("TEST3");

    // missing input for output nft
    let inputs = build_input_signing_data_most_basic_outputs(vec![(bech32_address, 1_000_000)]);
    let outputs = vec![build_nft_output(nft_id_1, bech32_address, 1_000_000)];

    match InputSelection::build(outputs, inputs, protocol_parameters.clone())
        .finish()
        .select()
    {
        Err(Error::UnfulfillableRequirement(req)) => {
            assert_eq!(req, Requirement::Nft(nft_id_1));
        }
        _ => panic!("Should return missing nft input"),
    }

    Ok(())
}
