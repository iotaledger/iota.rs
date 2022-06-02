// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{ops::Range, str::FromStr};

use bee_block::{
    address::Address,
    input::{UtxoInput, INPUT_COUNT_MAX},
    output::{
        unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeTokensBuilder, Output, OutputId,
        UnlockCondition,
    },
    payload::transaction::TransactionId,
};

use crate::{
    api::block_builder::ClientBlockBuilder, node_api::indexer::query_parameters::QueryParameter, secret::SecretManager,
    Client, Result,
};

/// Function to consolidate all funds and native tokens from a range of addresses to the address with the lowest index
/// in that range. Returns the address to which the funds got consolidated, if any were available
pub async fn consolidate_funds(
    client: &Client,
    secret_manager: &SecretManager,
    account_index: u32,
    address_range: Range<u32>,
) -> Result<String> {
    let addresses = client
        .get_addresses(secret_manager)
        .with_account_index(account_index)
        .with_range(address_range.clone())
        .finish()
        .await?;

    let consolidation_address = addresses[0].clone();

    let mut last_transfer_index = address_range.start;
    let offset = address_range.start;
    'consolidation: loop {
        let mut block_ids = Vec::new();
        // Iterate over addresses reversed so the funds end up on the first address in the range
        for (index, address) in addresses.iter().enumerate().rev() {
            let index = index as u32;
            // add the offset so the index matches the address index also for higher start indexes
            let index = index + offset;

            // Get output ids of outputs that can be controlled by this address without further unlock constraints
            let basic_outputs = client
                .get_address()
                .outputs(vec![
                    QueryParameter::Address(address.to_string()),
                    QueryParameter::HasExpirationCondition(false),
                    QueryParameter::HasTimelockCondition(false),
                    QueryParameter::HasStorageReturnCondition(false),
                ])
                .await?;

            let mut output_with_metadata = Vec::new();

            for output in basic_outputs.iter() {
                let (amount, _output_address) =
                    ClientBlockBuilder::get_output_amount_and_address(&output.output, None)?;
                output_with_metadata.push((output.clone(), amount));
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

            let outputs_chunks = output_with_metadata.chunks(INPUT_COUNT_MAX.into());

            for chunk in outputs_chunks {
                let mut block_builder = client.block().with_secret_manager(secret_manager);
                let mut total_amount = 0;
                let mut total_native_tokens = NativeTokensBuilder::new();

                for (input, amount) in chunk {
                    block_builder = block_builder.with_input(UtxoInput::from(OutputId::new(
                        TransactionId::from_str(&input.metadata.transaction_id)?,
                        input.metadata.output_index,
                    )?))?;

                    let output = Output::try_from(&input.output)?;

                    if let Some(native_tokens) = output.native_tokens() {
                        total_native_tokens.add_native_tokens(native_tokens.clone())?;
                    }
                    total_amount += amount;
                }

                let consolidation_output = BasicOutputBuilder::new_with_amount(total_amount)?
                    .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
                        Address::try_from_bech32(&consolidation_address)?.1,
                    )))
                    .with_native_tokens(total_native_tokens.finish()?)
                    .finish_output()?;

                let block = block_builder
                    .with_input_range(index..index + 1)
                    .with_outputs(vec![consolidation_output])?
                    .with_initial_address_index(0)
                    .finish()
                    .await?;
                block_ids.push(block.id());
            }
        }

        if block_ids.is_empty() {
            break 'consolidation;
        }
        // Wait for txs to get confirmed so we don't create conflicting txs
        for block_id in block_ids {
            let _ = client.retry_until_included(&block_id, None, None).await?;
        }
    }
    Ok(consolidation_address)
}
