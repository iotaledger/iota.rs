// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example consolidation --release

use iota::{
    bee_rest_api::types::dtos::{AddressDto, OutputDto},
    client::Result,
    node::{OutputType, OutputsOptions},
    Address, Client, Ed25519Address, MessageId, Seed,
};
extern crate dotenv;
use dotenv::dotenv;
use std::{env, str::FromStr};

/// In this example we will consolidate all funds in a range of addresses and send them to a single address

#[tokio::main]
async fn main() -> Result<()> {
    let address_consolidation_range = 0..100;
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com")?
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);

    let addresses = iota
        .get_addresses(&seed)
        .with_range(address_consolidation_range.clone())
        .finish()
        .await?;
    let consolidation_address = addresses[0].clone();

    let mut last_transfer_index = address_consolidation_range.start;
    let mut message_id: Option<MessageId> = None;
    'consolidation: loop {
        for (index, address) in addresses.iter().enumerate().rev() {
            // We request the different output types separated so we don't get problems with the dust protection,
            // since the maximum is 100 dust and when we add the signature locked single outptus first we will always
            // have > 1 Mi for the output
            let signature_locked_outputs = iota
                .get_address()
                .outputs(
                    &address,
                    OutputsOptions {
                        include_spent: false,
                        output_type: Some(OutputType::SignatureLockedSingle),
                    },
                )
                .await?;
            let dust_allowance_outputs = iota
                .get_address()
                .outputs(
                    &address,
                    OutputsOptions {
                        include_spent: false,
                        output_type: Some(OutputType::SignatureLockedDustAllowance),
                    },
                )
                .await?;

            let mut output_with_metadata = Vec::new();
            let all_outputs = [signature_locked_outputs, dust_allowance_outputs].concat();

            for out in all_outputs.iter() {
                let output_metadata = iota.get_output(out).await?;
                let (amount, _output_address, _check_treshold) =
                    get_output_amount_and_address(&output_metadata.output)?;
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
            let outputs_chunks = output_with_metadata.chunks(127);
            for chunk in outputs_chunks {
                let mut message_builder = iota.message().with_seed(&seed);
                let mut total_amount = 0;
                for (input, amount) in chunk {
                    message_builder = message_builder.with_input(input.clone());
                    total_amount += amount;
                }
                println!("Funds on index {}", index);

                let message = message_builder
                    .with_input_range(index..index + 1)
                    .with_output(&consolidation_address, total_amount)?
                    .with_initial_address_index(0)
                    .finish()
                    .await?;
                message_id.replace(message.id().0);
                println!(
                    "Transaction sent: https://explorer.iota.org/testnet/message/{}",
                    message_id.unwrap()
                );
            }
        }
        // Wait for the last tx to get confirmed so we don't create conflicting txs
        if let Some(message_id) = message_id {
            let _ = iota.retry_until_included(&message_id, None, None).await?;
        }
    }

    Ok(())
}

fn get_output_amount_and_address(output: &OutputDto) -> Result<(u64, Address, bool)> {
    match output {
        OutputDto::Treasury(_) => Err(iota::error::Error::OutputError("Treasury not allowed")),
        OutputDto::SignatureLockedSingle(ref r) => match &r.address {
            AddressDto::Ed25519(addr) => {
                let output_address = Address::from(Ed25519Address::from_str(&addr.address)?);
                Ok((r.amount, output_address, true))
            }
        },
        OutputDto::SignatureLockedDustAllowance(ref r) => match &r.address {
            AddressDto::Ed25519(addr) => {
                let output_address = Address::from(Ed25519Address::from_str(&addr.address)?);
                Ok((r.amount, output_address, false))
            }
        },
    }
}
