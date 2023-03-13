// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use iota_types::block::{
    address::Address,
    input::{UtxoInput, INPUT_COUNT_MAX},
    output::{
        unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeTokensBuilder, Output, OutputId,
        UnlockCondition,
    },
    payload::transaction::TransactionId,
};

use crate::{
    api::GetAddressesBuilderOptions, node_api::indexer::query_parameters::QueryParameter, secret::SecretManager,
    Client, Result,
};

impl Client {
    /// Function to consolidate all funds and native tokens from a range of addresses to the address with the lowest
    /// index in that range. Returns the address to which the funds got consolidated, if any were available
    pub async fn consolidate_funds(
        &self,
        secret_manager: &SecretManager,
        address_builder_options: GetAddressesBuilderOptions,
    ) -> Result<String> {
        let token_supply = self.get_token_supply().await?;
        let mut last_transfer_index = address_builder_options.range.as_ref().unwrap_or(&(0..1)).start;
        // use the start index as offset
        let offset = last_transfer_index;

        let addresses = self
            .get_addresses(secret_manager)
            .set_options(address_builder_options)?
            .finish()
            .await?;

        let consolidation_address = addresses[0].clone();

        'consolidation: loop {
            let mut block_ids = Vec::new();
            // Iterate over addresses reversed so the funds end up on the first address in the range
            for (index, address) in addresses.iter().enumerate().rev() {
                let index = index as u32;
                // add the offset so the index matches the address index also for higher start indexes
                let index = index + offset;

                // Get output ids of outputs that can be controlled by this address without further unlock constraints
                let output_ids_response = self
                    .basic_output_ids(vec![
                        QueryParameter::Address(address.to_string()),
                        QueryParameter::HasExpiration(false),
                        QueryParameter::HasTimelock(false),
                        QueryParameter::HasStorageDepositReturn(false),
                    ])
                    .await?;

                let basic_outputs_responses = self.get_outputs(output_ids_response.items).await?;

                if !basic_outputs_responses.is_empty() {
                    // If we reach the same index again
                    if last_transfer_index == index {
                        if basic_outputs_responses.len() < 2 {
                            break 'consolidation;
                        }
                    } else {
                        last_transfer_index = index;
                    }
                }

                let outputs_chunks = basic_outputs_responses.chunks(INPUT_COUNT_MAX.into());

                for chunk in outputs_chunks {
                    let mut block_builder = self.block().with_secret_manager(secret_manager);
                    let mut total_amount = 0;
                    let mut total_native_tokens = NativeTokensBuilder::new();

                    for output_response in chunk {
                        block_builder = block_builder.with_input(UtxoInput::from(OutputId::new(
                            TransactionId::from_str(&output_response.metadata.transaction_id)?,
                            output_response.metadata.output_index,
                        )?))?;

                        let output = Output::try_from_dto(&output_response.output, token_supply)?;

                        if let Some(native_tokens) = output.native_tokens() {
                            total_native_tokens.add_native_tokens(native_tokens.clone())?;
                        }
                        total_amount += output.amount();
                    }

                    let consolidation_output = BasicOutputBuilder::new_with_amount(total_amount)?
                        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(
                            Address::try_from_bech32(&consolidation_address)?.1,
                        )))
                        .with_native_tokens(total_native_tokens.finish()?)
                        .finish_output(token_supply)?;

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
                let _ = self.retry_until_included(&block_id, None, None).await?;
            }
        }
        Ok(consolidation_address)
    }
}
