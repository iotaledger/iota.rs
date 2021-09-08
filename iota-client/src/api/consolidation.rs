// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    api::message_builder::ClientMessageBuilder,
    node::{OutputType, OutputsOptions},
    Client, Result,
};
use bee_message::constants::INPUT_OUTPUT_COUNT_MAX;
use crypto::keys::slip10::Seed;
use std::ops::Range;

/// Function to consolidate all funds from a range of addresses to the address with the lowest index in that range
/// Returns the address to which the funds got consolidated, if any were available
pub async fn consolidate_funds(
    client: &Client,
    seed: &Seed,
    account_index: usize,
    address_range: Range<usize>,
) -> Result<String> {
    let addresses = client
        .get_addresses(seed)
        .with_account_index(account_index)
        .with_range(address_range.clone())
        .finish()
        .await?;
    let consolidation_address = addresses[0].clone();

    let mut last_transfer_index = address_range.start;
    let offset = address_range.start;
    'consolidation: loop {
        let mut message_ids = Vec::new();
        // Iterate over addresses reversed so the funds end up on the first address in the range
        for (index, address) in addresses.iter().enumerate().rev() {
            // add the offset so the index matches the address index also for higher start indexes
            let index = index + offset;
            // We request the different output types separated so we don't get problems with the dust protection,
            // since the maximum is 100 dust and when we add the signature locked single outptus first we will always
            // have > 1 Mi for the output
            let signature_locked_outputs = client
                .get_address()
                .outputs(
                    address,
                    OutputsOptions {
                        include_spent: false,
                        output_type: Some(OutputType::SignatureLockedSingle),
                    },
                )
                .await?;
            let dust_allowance_outputs = client
                .get_address()
                .outputs(
                    address,
                    OutputsOptions {
                        include_spent: false,
                        output_type: Some(OutputType::SignatureLockedDustAllowance),
                    },
                )
                .await?;

            let mut output_with_metadata = Vec::new();

            for out in signature_locked_outputs.iter().chain(dust_allowance_outputs.iter()) {
                let output_metadata = client.get_output(out).await?;
                let (amount, _output_address, _check_treshold) =
                    ClientMessageBuilder::get_output_amount_and_address(&output_metadata.output)?;
                output_with_metadata.push((out.clone(), amount));
            }

            if !output_with_metadata.is_empty() {
                // If we reach the same index again
                if last_transfer_index == index {
                    if output_with_metadata.len() < 2 {
                        break 'consolidation;
                    }
                } else {
                    last_transfer_index = index;
                }
            }

            let outputs_chunks = output_with_metadata.chunks(INPUT_OUTPUT_COUNT_MAX);

            for chunk in outputs_chunks {
                let mut message_builder = client.message().with_seed(seed);
                let mut total_amount = 0;
                for (input, amount) in chunk {
                    message_builder = message_builder.with_input(input.clone());
                    total_amount += amount;
                }

                let message = message_builder
                    .with_input_range(index..index + 1)
                    .with_output(&consolidation_address, total_amount)?
                    .with_initial_address_index(0)
                    .finish()
                    .await?;
                message_ids.push(message.id().0);
            }
        }

        if message_ids.is_empty() {
            break 'consolidation;
        }
        // Wait for txs to get confirmed so we don't create conflicting txs
        for message_id in message_ids {
            let _ = client.retry_until_included(&message_id, None, None).await?;
        }
    }
    Ok(consolidation_address)
}
