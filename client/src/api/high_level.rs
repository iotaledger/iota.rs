// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{collections::HashSet, str::FromStr};

use iota_types::{
    api::core::{dto::LedgerInclusionStateDto, response::OutputWithMetadataResponse},
    block::{
        input::{Input, UtxoInput, INPUT_COUNT_MAX},
        output::{Output, OutputId},
        parent::Parents,
        payload::{
            transaction::{TransactionEssence, TransactionId},
            Payload,
        },
        Block, BlockId,
    },
};

use crate::{
    api::{input_selection::Error as InputSelectionError, ClientBlockBuilder, GetAddressesBuilder},
    constants::{
        DEFAULT_RETRY_UNTIL_INCLUDED_INTERVAL, DEFAULT_RETRY_UNTIL_INCLUDED_MAX_AMOUNT, FIVE_MINUTES_IN_SECONDS,
    },
    error::{Error, Result},
    node_api::indexer::query_parameters::QueryParameter,
    secret::SecretManager,
    unix_timestamp_now, Client,
};

impl Client {
    /// Get the inputs of a transaction for the given transaction id.
    pub async fn inputs_from_transaction_id(
        &self,
        transaction_id: &TransactionId,
    ) -> Result<Vec<OutputWithMetadataResponse>> {
        let block = self.get_included_block(transaction_id).await?;

        let inputs = match block.payload() {
            Some(Payload::Transaction(t)) => match t.essence() {
                TransactionEssence::Regular(e) => e.inputs(),
            },
            _ => {
                unreachable!()
            }
        };

        let input_ids = inputs
            .iter()
            .map(|i| match i {
                Input::Utxo(input) => *input.output_id(),
                Input::Treasury(_) => {
                    unreachable!()
                }
            })
            .collect();

        self.get_outputs(input_ids).await
    }

    /// A generic send function for easily sending transaction or tagged data blocks.
    pub fn block(&self) -> ClientBlockBuilder<'_> {
        ClientBlockBuilder::new(self)
    }

    /// Return a list of addresses from a secret manager regardless of their validity.
    pub fn get_addresses<'a>(&'a self, secret_manager: &'a SecretManager) -> GetAddressesBuilder<'a> {
        GetAddressesBuilder::new(secret_manager).with_client(self)
    }

    /// Find all blocks by provided block IDs.
    pub async fn find_blocks(&self, block_ids: &[BlockId]) -> Result<Vec<Block>> {
        let mut blocks = Vec::new();

        // Use a `HashSet` to prevent duplicate block_ids.
        let mut block_ids_to_query = HashSet::<BlockId>::new();

        // Collect the `BlockId` in the HashSet.
        for block_id in block_ids {
            block_ids_to_query.insert(*block_id);
        }

        // Use `get_block()` API to get the `Block`.
        for block_id in block_ids_to_query {
            let block = self.get_block(&block_id).await?;
            blocks.push(block);
        }
        Ok(blocks)
    }

    /// Retries (promotes or reattaches) a block for provided block id. Block should only be
    /// retried only if they are valid and haven't been confirmed for a while.
    pub async fn retry(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        // Get the metadata to check if it needs to promote or reattach
        let block_metadata = self.get_block_metadata(block_id).await?;
        if block_metadata.should_promote.unwrap_or(false) {
            self.promote_unchecked(block_id).await
        } else if block_metadata.should_reattach.unwrap_or(false) {
            self.reattach_unchecked(block_id).await
        } else {
            Err(Error::NoNeedPromoteOrReattach(block_id.to_string()))
        }
    }

    /// Retries (promotes or reattaches) a block for provided block id until it's included (referenced by a
    /// milestone). Default interval is 5 seconds and max attempts is 40. Returns the included block at first position
    /// and additional reattached blocks
    pub async fn retry_until_included(
        &self,
        block_id: &BlockId,
        interval: Option<u64>,
        max_attempts: Option<u64>,
    ) -> Result<Vec<(BlockId, Block)>> {
        log::debug!("[retry_until_included]");
        // Attachments of the Block to check inclusion state
        let mut block_ids = vec![*block_id];
        // Reattached Blocks that get returned
        let mut blocks_with_id = Vec::new();
        for _ in 0..max_attempts.unwrap_or(DEFAULT_RETRY_UNTIL_INCLUDED_MAX_AMOUNT) {
            #[cfg(target_family = "wasm")]
            gloo_timers::future::TimeoutFuture::new(
                (interval.unwrap_or(DEFAULT_RETRY_UNTIL_INCLUDED_INTERVAL) * 1000)
                    .try_into()
                    .unwrap(),
            )
            .await;

            #[cfg(not(target_family = "wasm"))]
            tokio::time::sleep(std::time::Duration::from_secs(
                interval.unwrap_or(DEFAULT_RETRY_UNTIL_INCLUDED_INTERVAL),
            ))
            .await;

            // Check inclusion state for each attachment
            let block_ids_len = block_ids.len();
            let mut conflicting = false;
            for (index, block_id_) in block_ids.clone().iter().enumerate() {
                let block_metadata = self.get_block_metadata(block_id_).await?;
                if let Some(inclusion_state) = block_metadata.ledger_inclusion_state {
                    match inclusion_state {
                        LedgerInclusionStateDto::Included | LedgerInclusionStateDto::NoTransaction => {
                            // if original block, request it so we can return it on first position
                            if block_id == block_id_ {
                                let mut included_and_reattached_blocks =
                                    vec![(*block_id, self.get_block(block_id).await?)];
                                included_and_reattached_blocks.extend(blocks_with_id);
                                return Ok(included_and_reattached_blocks);
                            } else {
                                // Move included block to first position
                                blocks_with_id.rotate_left(index);
                                return Ok(blocks_with_id);
                            }
                        }
                        // only set it as conflicting here and don't return, because another reattached block could
                        // have the included transaction
                        LedgerInclusionStateDto::Conflicting => conflicting = true,
                    };
                }
                // Only reattach or promote latest attachment of the block
                if index == block_ids_len - 1 {
                    if block_metadata.should_promote.unwrap_or(false) {
                        // Safe to unwrap since we iterate over it
                        self.promote_unchecked(block_ids.last().unwrap()).await?;
                    } else if block_metadata.should_reattach.unwrap_or(false) {
                        // Safe to unwrap since we iterate over it
                        let reattached = self.reattach_unchecked(block_ids.last().unwrap()).await?;
                        block_ids.push(reattached.0);
                        blocks_with_id.push(reattached);
                    }
                }
            }
            // After we checked all our reattached blocks, check if the transaction got reattached in another block
            // and confirmed
            if conflicting {
                let block = self.get_block(block_id).await?;
                if let Some(Payload::Transaction(transaction_payload)) = block.payload() {
                    let included_block = self.get_included_block(&transaction_payload.id()).await?;
                    let mut included_and_reattached_blocks = vec![(included_block.id(), included_block)];
                    included_and_reattached_blocks.extend(blocks_with_id);
                    return Ok(included_and_reattached_blocks);
                }
            }
        }
        Err(Error::TangleInclusion(block_id.to_string()))
    }

    /// Function to find inputs from addresses for a provided amount (useful for offline signing), ignoring outputs with
    /// additional unlock conditions
    pub async fn find_inputs(&self, addresses: Vec<String>, amount: u64) -> Result<Vec<UtxoInput>> {
        // Get outputs from node and select inputs
        let mut available_outputs = Vec::new();

        for address in addresses {
            let output_ids_response = self
                .basic_output_ids(vec![
                    QueryParameter::Address(address.to_string()),
                    QueryParameter::HasExpiration(false),
                    QueryParameter::HasTimelock(false),
                    QueryParameter::HasStorageDepositReturn(false),
                ])
                .await?;

            available_outputs.extend(self.get_outputs(output_ids_response.items).await?);
        }

        let mut basic_outputs = Vec::new();
        let token_supply = self.get_token_supply().await?;

        for output_resp in available_outputs {
            let amount = Output::try_from_dto(&output_resp.output, token_supply)?.amount();
            basic_outputs.push((
                UtxoInput::new(
                    TransactionId::from_str(&output_resp.metadata.transaction_id)?,
                    output_resp.metadata.output_index,
                )?,
                amount,
            ));
        }
        basic_outputs.sort_by(|l, r| r.1.cmp(&l.1));

        let mut total_already_spent = 0;
        let mut selected_inputs = Vec::new();
        for (_offset, output_wrapper) in basic_outputs
            .into_iter()
            // Max inputs is 128
            .take(INPUT_COUNT_MAX.into())
            .enumerate()
        {
            // Break if we have enough funds and don't create dust for the remainder
            if total_already_spent == amount || total_already_spent >= amount {
                break;
            }
            selected_inputs.push(output_wrapper.0);
            total_already_spent += output_wrapper.1;
        }

        if total_already_spent < amount {
            return Err(InputSelectionError::InsufficientAmount {
                found: total_already_spent,
                required: amount,
            })?;
        }

        Ok(selected_inputs)
    }

    /// Find all outputs based on the requests criteria. This method will try to query multiple nodes if
    /// the request amount exceeds individual node limit.
    pub async fn find_outputs(
        &self,
        output_ids: &[OutputId],
        addresses: &[String],
    ) -> Result<Vec<OutputWithMetadataResponse>> {
        let mut output_responses = self.get_outputs(output_ids.to_vec()).await?;

        // Use `get_address()` API to get the address outputs first,
        // then collect the `UtxoInput` in the HashSet.
        for address in addresses {
            // Get output ids of outputs that can be controlled by this address without further unlock constraints
            let output_ids_response = self
                .basic_output_ids(vec![
                    QueryParameter::Address(address.to_string()),
                    QueryParameter::HasExpiration(false),
                    QueryParameter::HasTimelock(false),
                    QueryParameter::HasStorageDepositReturn(false),
                ])
                .await?;

            output_responses.extend(self.get_outputs(output_ids_response.items).await?);
        }

        Ok(output_responses.clone())
    }

    /// Reattaches blocks for provided block id. Blocks can be reattached only if they are valid and haven't been
    /// confirmed for a while.
    pub async fn reattach(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        let metadata = self.get_block_metadata(block_id).await?;
        if metadata.should_reattach.unwrap_or(false) {
            self.reattach_unchecked(block_id).await
        } else {
            Err(Error::NoNeedPromoteOrReattach(block_id.to_string()))
        }
    }

    /// Reattach a block without checking if it should be reattached
    pub async fn reattach_unchecked(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        // Get the Block object by the BlockID.
        let block = self.get_block(block_id).await?;
        let reattach_block = self.finish_block_builder(None, block.payload().cloned()).await?;

        // Post the modified
        let block_id = self.post_block_raw(&reattach_block).await?;
        // Get block if we use remote Pow, because the node will change parents and nonce
        let block = if self.get_local_pow() {
            reattach_block
        } else {
            self.get_block(&block_id).await?
        };
        Ok((block_id, block))
    }

    /// Promotes a block. The method should validate if a promotion is necessary through get_block. If not, the
    /// method should error out and should not allow unnecessary promotions.
    pub async fn promote(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        let metadata = self.get_block_metadata(block_id).await?;
        if metadata.should_promote.unwrap_or(false) {
            self.promote_unchecked(block_id).await
        } else {
            Err(Error::NoNeedPromoteOrReattach(block_id.to_string()))
        }
    }

    /// Promote a block without checking if it should be promoted
    pub async fn promote_unchecked(&self, block_id: &BlockId) -> Result<(BlockId, Block)> {
        // Create a new block (zero value block) for which one tip would be the actual block.
        let mut tips = self.get_tips().await?;
        if let Some(tip) = tips.first_mut() {
            *tip = *block_id;
        }

        let promote_block = self.finish_block_builder(Some(Parents::new(tips)?), None).await?;

        let block_id = self.post_block_raw(&promote_block).await?;
        // Get block if we use remote Pow, because the node will change parents and nonce.
        let block = if self.get_local_pow() {
            promote_block
        } else {
            self.get_block(&block_id).await?
        };
        Ok((block_id, block))
    }

    /// Returns the local time checked with the timestamp of the latest milestone, if the difference is larger than 5
    /// minutes an error is returned to prevent locking outputs by accident for a wrong time.
    pub async fn get_time_checked(&self) -> Result<u32> {
        let current_time = unix_timestamp_now();

        let network_info = self.get_network_info().await?;

        if let Some(latest_ms_timestamp) = network_info.latest_milestone_timestamp {
            // Check the local time is in the range of +-5 minutes of the node to prevent locking funds by accident
            if !(latest_ms_timestamp - FIVE_MINUTES_IN_SECONDS..latest_ms_timestamp + FIVE_MINUTES_IN_SECONDS)
                .contains(&current_time)
            {
                return Err(Error::TimeNotSynced {
                    current_time,
                    milestone_timestamp: latest_ms_timestamp,
                });
            }
        }

        Ok(current_time)
    }
}
