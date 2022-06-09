// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{any::Any, panic::AssertUnwindSafe};

use backtrace::Backtrace;
use bee_block::{
    address::dto::AddressDto,
    input::dto::UtxoInputDto,
    output::{AliasId, FoundryId, NftId},
    payload::{
        dto::{MilestonePayloadDto, PayloadDto},
        Payload, TransactionPayload,
    },
    Block as BeeBlock, BlockDto,
};
use futures::{Future, FutureExt};
use tokio::sync::mpsc::UnboundedSender;
use zeroize::Zeroize;

use crate::{
    api::{PreparedTransactionData, PreparedTransactionDataDto},
    message_interface::{client_method::ClientMethod, message::Message, output_builder::{build_alias_output, build_basic_output, build_foundry_output, build_nft_output}, response::Response},
    request_funds_from_faucet,
    secret::SecretManager,
    Client, Result,
};

fn panic_to_response_message(panic: Box<dyn Any>) -> Response {
    let msg = if let Some(message) = panic.downcast_ref::<String>() {
        format!("Internal error: {}", message)
    } else if let Some(message) = panic.downcast_ref::<&str>() {
        format!("Internal error: {}", message)
    } else {
        "Internal error".to_string()
    };
    let current_backtrace = Backtrace::new();
    Response::Panic(format!("{}\n\n{:?}", msg, current_backtrace))
}

async fn convert_async_panics<F>(f: impl FnOnce() -> F) -> Result<Response>
where
    F: Future<Output = Result<Response>>,
{
    match AssertUnwindSafe(f()).catch_unwind().await {
        Ok(result) => result,
        Err(panic) => Ok(panic_to_response_message(panic)),
    }
}

/// The Client message handler.
pub struct ClientMessageHandler {
    /// The Client
    pub client: Client,
}

impl ClientMessageHandler {
    /// Creates a new instance of the message handler with the default client manager.
    pub async fn new() -> Result<Self> {
        let instance = Self {
            client: Client::builder().finish().await?,
        };
        Ok(instance)
    }

    /// Creates a new instance of the message handler with the specified client.
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// Handle messages
    pub async fn handle(&self, mut message: Message, response_tx: UnboundedSender<Response>) {
        let result: Result<Response> = match message {
            Message::CallClientMethod(ref method) => {
                convert_async_panics(|| async { self.call_client_method(method).await }).await
            }
        };

        // Zeroize secrets as soon as their missions are finished.
        match &mut message {
            #[cfg(feature = "stronghold")]
            Message::CallClientMethod(ClientMethod::StoreMnemonic { mnemonic, .. }) => mnemonic.zeroize(),
            Message::CallClientMethod(ClientMethod::MnemonicToHexSeed { mnemonic }) => mnemonic.zeroize(),

            // SecretManagerDto impl ZeroizeOnDrop, so we don't have to call zeroize() here.
            _ => (),
        };

        let response = match result {
            Ok(r) => r,
            Err(e) => Response::Error(e),
        };

        let _ = response_tx.send(response);
    }

    // If cfg(not(feature = "stronghold")) then secret_manager doesn't necessarily to be mutable, but otherwise it has
    // to be. Instead of rendering the code messy just because of this, we just allow unused mutable variables.
    #[allow(unused_mut)]
    async fn call_client_method(&self, method: &ClientMethod) -> Result<Response> {
        match method {
            ClientMethod::BuildAliasOutput {
                amount,
                native_tokens,
                alias_id,
                state_index,
                state_metadata,
                foundry_counter,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output_dto = build_alias_output(
                    &self.client,
                    amount.clone(),
                    native_tokens.clone(),
                    alias_id,
                    *state_index,
                    state_metadata.clone(),
                    *foundry_counter,
                    unlock_conditions.to_vec(),
                    features.clone(),
                    immutable_features.clone(),
                )
                .await?;
                Ok(Response::BuiltOutput(output_dto))
            }
            ClientMethod::BuildBasicOutput {
                amount,
                native_tokens,
                unlock_conditions,
                features,
            } => {
                let output_dto = build_basic_output(
                    &self.client,
                    amount.clone(),
                    native_tokens.clone(),
                    unlock_conditions.to_vec(),
                    features.clone(),
                )
                .await?;
                Ok(Response::BuiltOutput(output_dto))
            }
            ClientMethod::BuildFoundryOutput {
                amount,
                native_tokens,
                serial_number,
                token_scheme,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output_dto = build_foundry_output(
                    &self.client,
                    amount.clone(),
                    native_tokens.clone(),
                    *serial_number,
                    token_scheme,
                    unlock_conditions.to_vec(),
                    features.clone(),
                    immutable_features.clone(),
                )
                .await?;
                Ok(Response::BuiltOutput(output_dto))
            }
            ClientMethod::BuildNftOutput {
                amount,
                native_tokens,
                nft_id,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output_dto = build_nft_output(
                    &self.client,
                    amount.clone(),
                    native_tokens.clone(),
                    nft_id,
                    unlock_conditions.to_vec(),
                    features.clone(),
                    immutable_features.clone(),
                )
                .await?;
                Ok(Response::BuiltOutput(output_dto))
            }
            ClientMethod::GenerateAddresses {
                secret_manager,
                options,
            } => {
                let mut secret_manager: SecretManager = secret_manager.try_into()?;
                // If we use stronghold we need to read the snapshot in case it hasn't been done already
                #[cfg(feature = "stronghold")]
                if let SecretManager::Stronghold(stronghold_secret_manager) = &mut secret_manager {
                    stronghold_secret_manager.read_stronghold_snapshot().await?;
                }
                let addresses = self
                    .client
                    .get_addresses(&secret_manager)
                    .set_options(options.clone())?
                    .finish()
                    .await?;
                Ok(Response::GeneratedAddresses(addresses))
            }
            ClientMethod::GenerateBlock {
                secret_manager,
                options,
            } => {
                // Prepare transaction
                let mut transaction_builder = self.client.block();

                let secret_manager: Option<SecretManager> = match secret_manager {
                    Some(secret_manager) => {
                        let mut secret_manager = secret_manager.try_into()?;
                        // If we use stronghold we need to read the snapshot in case it hasn't been done already
                        #[cfg(feature = "stronghold")]
                        if let SecretManager::Stronghold(stronghold_secret_manager) = &mut secret_manager {
                            stronghold_secret_manager.read_stronghold_snapshot().await?;
                        }
                        Some(secret_manager)
                    }
                    None => None,
                };

                if let Some(secret_manager) = &secret_manager {
                    transaction_builder = transaction_builder.with_secret_manager(secret_manager);
                }

                if let Some(options) = options {
                    transaction_builder = transaction_builder.set_options(options.clone())?;
                }

                Ok(Response::GeneratedBlock(BlockDto::from(
                    &transaction_builder.finish().await?,
                )))
            }
            ClientMethod::GetNode => Ok(Response::Node(self.client.get_node().await?)),
            ClientMethod::GetNetworkInfo => Ok(Response::NetworkInfo(self.client.get_network_info().await?)),
            ClientMethod::GetNetworkId => Ok(Response::NetworkId(self.client.get_network_id().await?)),
            ClientMethod::GetBech32Hrp => Ok(Response::Bech32Hrp(self.client.get_bech32_hrp().await?)),
            ClientMethod::GetMinPoWScore => Ok(Response::MinPoWScore(self.client.get_min_pow_score().await?)),
            ClientMethod::GetTipsInterval => Ok(Response::TipsInterval(self.client.get_tips_interval().await)),
            ClientMethod::GetLocalPoW => Ok(Response::LocalPoW(self.client.get_local_pow().await)),
            ClientMethod::GetFallbackToLocalPoW => Ok(Response::FallbackToLocalPoW(
                self.client.get_fallback_to_local_pow().await,
            )),
            ClientMethod::PrepareTransaction {
                secret_manager,
                options,
            } => {
                let mut block_builder = self.client.block();

                let secret_manager = match secret_manager {
                    Some(secret_manager) => {
                        let mut secret_manager = secret_manager.try_into()?;
                        // If we use stronghold we need to read the snapshot in case it hasn't been done already
                        #[cfg(feature = "stronghold")]
                        if let SecretManager::Stronghold(stronghold_secret_manager) = &mut secret_manager {
                            stronghold_secret_manager.read_stronghold_snapshot().await?;
                        }
                        Some(secret_manager)
                    }
                    None => None,
                };

                if let Some(secret_manager) = &secret_manager {
                    block_builder = block_builder.with_secret_manager(secret_manager);
                }

                if let Some(options) = options {
                    block_builder = block_builder.set_options(options.clone())?;
                }

                Ok(Response::PreparedTransactionData(PreparedTransactionDataDto::from(
                    &block_builder.prepare_transaction().await?,
                )))
            }
            ClientMethod::SignTransaction {
                secret_manager,
                prepared_transaction_data,
            } => {
                let mut block_builder = self.client.block();

                let mut secret_manager: SecretManager = secret_manager.try_into()?;
                // If we use stronghold we need to read the snapshot in case it hasn't been done already
                #[cfg(feature = "stronghold")]
                if let SecretManager::Stronghold(stronghold_secret_manager) = &mut secret_manager {
                    stronghold_secret_manager.read_stronghold_snapshot().await?;
                }

                block_builder = block_builder.with_secret_manager(&secret_manager);

                Ok(Response::SignedTransaction(PayloadDto::from(
                    &block_builder
                        .sign_transaction(PreparedTransactionData::try_from(prepared_transaction_data)?)
                        .await?,
                )))
            }
            #[cfg(feature = "stronghold")]
            ClientMethod::StoreMnemonic {
                secret_manager,
                mnemonic,
            } => {
                let mut secret_manager: SecretManager = secret_manager.try_into()?;
                if let SecretManager::Stronghold(secret_manager) = &mut secret_manager {
                    secret_manager.store_mnemonic(mnemonic.to_string()).await?;
                } else {
                    return Err(crate::Error::SecretManagerMismatch);
                }
                Ok(Response::Ok(()))
            }
            ClientMethod::SubmitPayload { payload_dto } => {
                let block_builder = self.client.block();

                Ok(Response::GeneratedBlock(BlockDto::from(
                    &block_builder
                        .finish_block(Some(Payload::try_from(payload_dto)?))
                        .await?,
                )))
            }
            #[cfg(not(feature = "wasm"))]
            ClientMethod::UnsyncedNodes => Ok(Response::UnsyncedNodes(
                self.client.unsynced_nodes().await.into_iter().cloned().collect(),
            )),
            ClientMethod::GetHealth { url } => Ok(Response::Health(self.client.get_health(url).await?)),
            ClientMethod::GetNodeInfo { url, auth } => {
                Ok(Response::NodeInfo(Client::get_node_info(url, auth.clone()).await?))
            }
            ClientMethod::GetInfo => Ok(Response::Info(self.client.get_info().await?)),
            ClientMethod::GetPeers => Ok(Response::Peers(self.client.get_peers().await?)),
            ClientMethod::GetTips => Ok(Response::Tips(self.client.get_tips().await?)),
            ClientMethod::PostBlockRaw { block } => Ok(Response::PostBlockSuccessful(
                self.client.post_block_raw(&BeeBlock::try_from(block)?).await?,
            )),
            ClientMethod::PostBlock { block } => Ok(Response::PostBlockSuccessful(
                self.client.post_block(&BeeBlock::try_from(block)?).await?,
            )),
            ClientMethod::GetBlock { block_id } => {
                Ok(Response::Block(BlockDto::from(&self.client.get_block(block_id).await?)))
            }
            ClientMethod::GetBlockMetadata { block_id } => {
                Ok(Response::BlockMetadata(self.client.get_block_metadata(block_id).await?))
            }
            ClientMethod::GetBlockRaw { block_id } => {
                Ok(Response::BlockRaw(self.client.get_block_raw(block_id).await?))
            }
            ClientMethod::GetBlockChildren { block_id } => {
                Ok(Response::BlockChildren(self.client.get_block_children(block_id).await?))
            }
            ClientMethod::GetOutput { output_id } => Ok(Response::Output(self.client.get_output(output_id).await?)),
            ClientMethod::GetOutputMetadata { output_id } => Ok(Response::OutputMetadata(
                self.client.get_output_metadata(output_id).await?,
            )),
            ClientMethod::GetMilestoneById { milestone_id } => Ok(Response::Milestone(MilestonePayloadDto::from(
                &self.client.get_milestone_by_id(milestone_id).await?,
            ))),
            ClientMethod::GetMilestoneByIdRaw { milestone_id } => Ok(Response::MilestoneRaw(
                self.client.get_milestone_by_id_raw(milestone_id).await?,
            )),
            ClientMethod::GetMilestoneByIndex { index } => Ok(Response::Milestone(MilestonePayloadDto::from(
                &self.client.get_milestone_by_index(*index).await?,
            ))),
            ClientMethod::GetMilestoneByIndexRaw { index } => Ok(Response::MilestoneRaw(
                self.client.get_milestone_by_index_raw(*index).await?,
            )),
            ClientMethod::GetUtxoChangesById { milestone_id } => Ok(Response::MilestoneUtxoChanges(
                self.client.get_utxo_changes_by_id(milestone_id).await?,
            )),
            ClientMethod::GetUtxoChangesByIndex { index } => Ok(Response::MilestoneUtxoChanges(
                self.client.get_utxo_changes_by_index(*index).await?,
            )),
            ClientMethod::GetReceipts => Ok(Response::Receipts(self.client.get_receipts().await?)),
            ClientMethod::GetReceiptsMigratedAt { milestone_index } => Ok(Response::ReceiptsMigratedAtMilestone(
                self.client.get_receipts_migrated_at(*milestone_index).await?,
            )),
            ClientMethod::GetTreasury => Ok(Response::Treasury(self.client.get_treasury().await?)),
            ClientMethod::GetIncludedBlock { transaction_id } => Ok(Response::IncludedBlock(BlockDto::from(
                &self.client.get_included_block(transaction_id).await?,
            ))),
            ClientMethod::BasicOutputIds { query_parameters } => Ok(Response::OutputIds(
                self.client.basic_output_ids(query_parameters.clone()).await?,
            )),
            ClientMethod::AliasOutputIds { query_parameters } => Ok(Response::OutputIds(
                self.client.alias_output_ids(query_parameters.clone()).await?,
            )),
            ClientMethod::AliasOutputId { alias_id } => {
                Ok(Response::OutputId(self.client.alias_output_id(*alias_id).await?))
            }
            ClientMethod::NftOutputIds { query_parameters } => Ok(Response::OutputIds(
                self.client.nft_output_ids(query_parameters.clone()).await?,
            )),
            ClientMethod::NftOutputId { nft_id } => Ok(Response::OutputId(self.client.nft_output_id(*nft_id).await?)),
            ClientMethod::FoundryOutputIds { query_parameters } => Ok(Response::OutputIds(
                self.client.foundry_output_ids(query_parameters.clone()).await?,
            )),
            ClientMethod::FoundryOutputId { foundry_id } => {
                Ok(Response::OutputId(self.client.foundry_output_id(*foundry_id).await?))
            }
            ClientMethod::GetOutputs { output_ids } => {
                Ok(Response::Outputs(self.client.get_outputs(output_ids.clone()).await?))
            }
            ClientMethod::TryGetOutputs { output_ids } => Ok(Response::Outputs(
                self.client.try_get_outputs(output_ids.clone()).await?,
            )),
            ClientMethod::FindBlocks { block_ids } => Ok(Response::Blocks(
                self.client
                    .find_blocks(block_ids)
                    .await?
                    .iter()
                    .map(BlockDto::from)
                    .collect(),
            )),
            ClientMethod::Retry { block_id } => {
                let (block_id, block) = self.client.retry(block_id).await?;
                Ok(Response::RetrySuccessful((block_id, BlockDto::from(&block))))
            }
            ClientMethod::RetryUntilIncluded {
                block_id,
                interval,
                max_attempts,
            } => {
                let res = self
                    .client
                    .retry_until_included(block_id, *interval, *max_attempts)
                    .await?;
                let res = res
                    .into_iter()
                    .map(|(block_id, block)| (block_id, BlockDto::from(&block)))
                    .collect();
                Ok(Response::RetryUntilIncludedSuccessful(res))
            }
            ClientMethod::ConsolidateFunds {
                secret_manager,
                account_index,
                address_range,
            } => {
                let mut secret_manager: SecretManager = secret_manager.try_into()?;
                // If we use stronghold we need to read the snapshot in case it hasn't been done already
                #[cfg(feature = "stronghold")]
                if let SecretManager::Stronghold(stronghold_secret_manager) = &mut secret_manager {
                    stronghold_secret_manager.read_stronghold_snapshot().await?;
                }
                Ok(Response::ConsolidatedFunds(
                    self.client
                        .consolidate_funds(&secret_manager, *account_index, address_range.clone())
                        .await?,
                ))
            }
            ClientMethod::FindInputs { addresses, amount } => Ok(Response::Inputs(
                self.client
                    .find_inputs(addresses.clone(), *amount)
                    .await?
                    .iter()
                    .map(UtxoInputDto::from)
                    .collect(),
            )),
            ClientMethod::FindOutputs { output_ids, addresses } => Ok(Response::Outputs(
                self.client.find_outputs(output_ids, addresses).await?,
            )),
            ClientMethod::Reattach { block_id } => {
                let (block_id, block) = self.client.reattach(block_id).await?;
                Ok(Response::Reattached((block_id, BlockDto::from(&block))))
            }
            ClientMethod::ReattachUnchecked { block_id } => {
                let (block_id, block) = self.client.reattach_unchecked(block_id).await?;
                Ok(Response::Reattached((block_id, BlockDto::from(&block))))
            }
            ClientMethod::Promote { block_id } => {
                let (block_id, block) = self.client.promote(block_id).await?;
                Ok(Response::Promoted((block_id, BlockDto::from(&block))))
            }
            ClientMethod::PromoteUnchecked { block_id } => {
                let (block_id, block) = self.client.promote_unchecked(block_id).await?;
                Ok(Response::Promoted((block_id, BlockDto::from(&block))))
            }
            ClientMethod::Bech32ToHex { bech32 } => Ok(Response::Bech32ToHex(Client::bech32_to_hex(bech32)?)),
            ClientMethod::HexToBech32 { hex, bech32_hrp } => Ok(Response::HexToBech32(
                self.client.hex_to_bech32(hex, bech32_hrp.as_deref()).await?,
            )),
            ClientMethod::HexPublicKeyToBech32Address { hex, bech32_hrp } => Ok(Response::HexToBech32(
                self.client
                    .hex_public_key_to_bech32_address(hex, bech32_hrp.as_deref())
                    .await?,
            )),
            ClientMethod::ParseBech32Address { address } => Ok(Response::ParsedBech32Address(AddressDto::from(
                &Client::parse_bech32_address(address)?,
            ))),
            ClientMethod::IsAddressValid { address } => Ok(Response::IsAddressValid(Client::is_address_valid(address))),
            ClientMethod::GenerateMnemonic => Ok(Response::GeneratedMnemonic(Client::generate_mnemonic()?)),
            ClientMethod::MnemonicToHexSeed { mnemonic } => {
                Ok(Response::MnemonicHexSeed(Client::mnemonic_to_hex_seed(mnemonic)?))
            }
            ClientMethod::BlockId { block } => {
                let block = BeeBlock::try_from(block)?;
                Ok(Response::BlockId(block.id()))
            }
            ClientMethod::TransactionId { payload } => {
                let payload = TransactionPayload::try_from(payload)?;
                Ok(Response::TransactionId(payload.id()))
            }
            ClientMethod::ComputeAliasId { output_id } => Ok(Response::AliasId(AliasId::from(*output_id))),
            ClientMethod::ComputeNftId { output_id } => Ok(Response::NftId(NftId::from(*output_id))),
            ClientMethod::ComputeFoundryId {
                alias_address,
                serial_number,
                token_scheme_kind,
            } => Ok(Response::FoundryId(FoundryId::build(
                alias_address,
                *serial_number,
                *token_scheme_kind,
            ))),
            ClientMethod::Faucet { url, address } => {
                Ok(Response::Faucet(request_funds_from_faucet(url, address).await?))
            }
        }
    }
}
