// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{any::Any, panic::AssertUnwindSafe};

use backtrace::Backtrace;
use futures::{Future, FutureExt};
use iota_types::block::{
    address::dto::AddressDto,
    input::dto::UtxoInputDto,
    output::{
        dto::{OutputBuilderAmountDto, OutputDto, RentStructureDto},
        AliasId, AliasOutput, BasicOutput, FoundryId, FoundryOutput, NftId, NftOutput, Output,
    },
    payload::{
        dto::{MilestonePayloadDto, PayloadDto},
        transaction::TransactionEssence,
        Payload, TransactionPayload,
    },
    protocol::dto::ProtocolParametersDto,
    unlock::Unlock,
    Block, BlockDto, DtoError,
};
use zeroize::Zeroize;
#[cfg(feature = "mqtt")]
use {
    crate::mqtt::{MqttPayload, Topic},
    iota_types::block::payload::milestone::option::dto::ReceiptMilestoneOptionDto,
};

#[cfg(feature = "ledger_nano")]
use crate::secret::ledger_nano::LedgerSecretManager;
use crate::{
    api::{PreparedTransactionData, PreparedTransactionDataDto, RemainderData},
    message_interface::{message::Message, response::Response},
    request_funds_from_faucet,
    secret::{types::InputSigningData, SecretManage, SecretManager},
    Client, Result,
};

fn panic_to_response_message(panic: Box<dyn Any>) -> Response {
    let msg = panic.downcast_ref::<String>().map_or_else(
        || {
            panic.downcast_ref::<&str>().map_or_else(
                || "Internal error".to_string(),
                |message| format!("Internal error: {message}"),
            )
        },
        |message| format!("Internal error: {message}"),
    );
    let current_backtrace = Backtrace::new();
    Response::Panic(format!("{msg}\n\n{current_backtrace:?}"))
}

#[cfg(not(target_family = "wasm"))]
async fn convert_async_panics<F>(f: impl FnOnce() -> F + Send) -> Result<Response>
where
    F: Future<Output = Result<Response>> + Send,
{
    AssertUnwindSafe(f())
        .catch_unwind()
        .await
        .unwrap_or_else(|panic| Ok(panic_to_response_message(panic)))
}

#[cfg(target_family = "wasm")]
#[allow(clippy::future_not_send)]
async fn convert_async_panics<F>(f: impl FnOnce() -> F) -> Result<Response>
where
    F: Future<Output = Result<Response>>,
{
    AssertUnwindSafe(f())
        .catch_unwind()
        .await
        .unwrap_or_else(|panic| Ok(panic_to_response_message(panic)))
}

/// The Client message handler.
pub struct ClientMessageHandler {
    /// The Client
    pub client: Client,
}

impl ClientMessageHandler {
    /// Creates a new instance of the message handler with the default client manager.
    pub fn new() -> Result<Self> {
        let instance = Self {
            client: Client::builder().finish()?,
        };
        Ok(instance)
    }

    /// Creates a new instance of the message handler with the specified client.
    pub fn with_client(client: Client) -> Self {
        Self { client }
    }

    /// Listen to MQTT events
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    pub async fn listen<F>(&self, topics: Vec<Topic>, handler: F)
    where
        F: Fn(String) + 'static + Clone + Send + Sync,
    {
        self.client
            .subscribe(topics, move |topic_event| {
                #[derive(Serialize)]
                struct MqttResponse {
                    topic: String,
                    payload: String,
                }
                // convert types to DTOs
                let payload = match &topic_event.payload {
                    MqttPayload::Json(val) => {
                        serde_json::to_string(&val).expect("failed to serialize MqttPayload::Json")
                    }
                    MqttPayload::Block(block) => {
                        serde_json::to_string(&BlockDto::from(block)).expect("failed to serialize MqttPayload::Block")
                    }
                    MqttPayload::MilestonePayload(ms) => serde_json::to_string(&MilestonePayloadDto::from(ms))
                        .expect("failed to serialize MqttPayload::MilestonePayload"),
                    MqttPayload::Receipt(receipt) => serde_json::to_string(&ReceiptMilestoneOptionDto::from(receipt))
                        .expect("failed to serialize MqttPayload::Receipt"),
                };
                let response = MqttResponse {
                    topic: topic_event.topic.clone(),
                    payload,
                };

                handler(serde_json::to_string(&response).expect("failed to serialize MQTT response"))
            })
            .await
            .expect("failed to listen to MQTT events");
    }

    /// Send a message.
    pub async fn send_message(&self, message: Message) -> Response {
        match &message {
            // Don't log secrets
            Message::GenerateAddresses {
                secret_manager: _,
                options,
            } => {
                log::debug!("Response: GenerateAddresses{{ secret_manager: <omitted>, options: {options:?} }}")
            }
            Message::BuildAndPostBlock {
                secret_manager: _,
                options,
            } => {
                log::debug!("Response: BuildAndPostBlock{{ secret_manager: <omitted>, options: {options:?} }}")
            }
            Message::PrepareTransaction {
                secret_manager: _,
                options,
            } => {
                log::debug!("Response: PrepareTransaction{{ secret_manager: <omitted>, options: {options:?} }}")
            }
            Message::SignTransaction {
                secret_manager: _,
                prepared_transaction_data,
            } => {
                log::debug!(
                    "Response: SignTransaction{{ secret_manager: <omitted>, prepared_transaction_data: {prepared_transaction_data:?} }}"
                )
            }
            #[cfg(feature = "stronghold")]
            Message::StoreMnemonic { .. } => {
                log::debug!("Response: StoreMnemonic{{ <omitted> }}")
            }
            Message::ConsolidateFunds {
                secret_manager: _,
                generate_addresses_options,
            } => {
                log::debug!(
                    "Response: ConsolidateFunds{{ secret_manager: <omitted>, generate_addresses_options: {generate_addresses_options:?} }}"
                )
            }
            Message::MnemonicToHexSeed { .. } => {
                log::debug!("Response: MnemonicToHexSeed{{ <omitted> }}")
            }
            _ => log::debug!("Message: {:?}", message),
        }

        let result = convert_async_panics(|| async { self.handle_message(message).await }).await;

        let response = match result {
            Ok(r) => r,
            Err(e) => Response::Error(e),
        };

        match response {
            // Don't log secrets
            Response::GeneratedMnemonic { .. } => {
                log::debug!("Response: GeneratedMnemonic(<omitted>)")
            }
            Response::MnemonicHexSeed { .. } => {
                log::debug!("Response: MnemonicHexSeed(<omitted>)")
            }
            _ => log::debug!("Response: {:?}", response),
        }

        response
    }

    // If cfg(not(feature = "stronghold")) then secret_manager doesn't necessarily to be mutable, but otherwise it has
    // to be. Instead of rendering the code messy just because of this, we just allow unused mutable variables.
    #[allow(unused_mut)]
    async fn handle_message(&self, message: Message) -> Result<Response> {
        match message {
            Message::BuildAliasOutput {
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
                let output = Output::from(AliasOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.client.get_rent_structure().await?)
                    },
                    native_tokens,
                    &alias_id,
                    state_index,
                    state_metadata.map(prefix_hex::decode).transpose()?,
                    foundry_counter,
                    unlock_conditions,
                    features,
                    immutable_features,
                    self.client.get_token_supply().await?,
                )?);

                Ok(Response::BuiltOutput(OutputDto::from(&output)))
            }
            Message::BuildBasicOutput {
                amount,
                native_tokens,
                unlock_conditions,
                features,
            } => {
                let output = Output::from(BasicOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.client.get_rent_structure().await?)
                    },
                    native_tokens,
                    unlock_conditions,
                    features,
                    self.client.get_token_supply().await?,
                )?);

                Ok(Response::BuiltOutput(OutputDto::from(&output)))
            }
            Message::BuildFoundryOutput {
                amount,
                native_tokens,
                serial_number,
                token_scheme,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output = Output::from(FoundryOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.client.get_rent_structure().await?)
                    },
                    native_tokens,
                    serial_number,
                    &token_scheme,
                    unlock_conditions,
                    features,
                    immutable_features,
                    self.client.get_token_supply().await?,
                )?);

                Ok(Response::BuiltOutput(OutputDto::from(&output)))
            }
            Message::BuildNftOutput {
                amount,
                native_tokens,
                nft_id,
                unlock_conditions,
                features,
                immutable_features,
            } => {
                let output = Output::from(NftOutput::try_from_dtos(
                    if let Some(amount) = amount {
                        OutputBuilderAmountDto::Amount(amount)
                    } else {
                        OutputBuilderAmountDto::MinimumStorageDeposit(self.client.get_rent_structure().await?)
                    },
                    native_tokens,
                    &nft_id,
                    unlock_conditions,
                    features,
                    immutable_features,
                    self.client.get_token_supply().await?,
                )?);

                Ok(Response::BuiltOutput(OutputDto::from(&output)))
            }
            Message::GenerateAddresses {
                secret_manager,
                options,
            } => {
                let secret_manager = (&secret_manager).try_into()?;
                let addresses = self
                    .client
                    .get_addresses(&secret_manager)
                    .set_options(options)?
                    .finish()
                    .await?;
                Ok(Response::GeneratedAddresses(addresses))
            }
            Message::BuildAndPostBlock {
                secret_manager,
                options,
            } => {
                // Prepare transaction
                let mut block_builder = self.client.block();

                let secret_manager = match secret_manager {
                    Some(secret_manager) => Some((&secret_manager).try_into()?),
                    None => None,
                };

                if let Some(secret_manager) = &secret_manager {
                    block_builder = block_builder.with_secret_manager(secret_manager);
                }

                if let Some(options) = options {
                    block_builder = block_builder.set_options(options).await?;
                }

                let block = block_builder.finish().await?;
                let block_id = block.id();

                Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
            }
            #[cfg(feature = "mqtt")]
            Message::ClearListeners { topics } => {
                self.client.unsubscribe(topics).await?;
                Ok(Response::Ok)
            }
            Message::GetNode => Ok(Response::Node(self.client.get_node()?)),
            Message::GetNetworkInfo => Ok(Response::NetworkInfo(self.client.get_network_info().await?.into())),
            Message::GetNetworkId => Ok(Response::NetworkId(self.client.get_network_id().await?)),
            Message::GetBech32Hrp => Ok(Response::Bech32Hrp(self.client.get_bech32_hrp().await?)),
            Message::GetMinPowScore => Ok(Response::MinPowScore(self.client.get_min_pow_score().await?)),
            Message::GetTipsInterval => Ok(Response::TipsInterval(self.client.get_tips_interval())),
            Message::GetProtocolParameters => {
                let params = self.client.get_protocol_parameters().await?;
                let protocol_response = ProtocolParametersDto {
                    protocol_version: params.protocol_version(),
                    network_name: params.network_name().to_string(),
                    bech32_hrp: params.bech32_hrp().to_string(),
                    min_pow_score: params.min_pow_score(),
                    below_max_depth: params.below_max_depth(),
                    rent_structure: RentStructureDto {
                        v_byte_cost: params.rent_structure().byte_cost(),
                        v_byte_factor_key: params.rent_structure().byte_factor_key(),
                        v_byte_factor_data: params.rent_structure().byte_factor_data(),
                    },
                    token_supply: params.token_supply().to_string(),
                };
                Ok(Response::ProtocolParameters(protocol_response))
            }
            Message::GetLocalPow => Ok(Response::LocalPow(self.client.get_local_pow())),
            Message::GetFallbackToLocalPow => Ok(Response::FallbackToLocalPow(self.client.get_fallback_to_local_pow())),
            #[cfg(feature = "ledger_nano")]
            Message::GetLedgerNanoStatus { is_simulator } => {
                let ledger_nano = LedgerSecretManager::new(is_simulator);

                Ok(Response::LedgerNanoStatus(ledger_nano.get_ledger_nano_status().await))
            }
            Message::PrepareTransaction {
                secret_manager,
                options,
            } => {
                let mut block_builder = self.client.block();

                let secret_manager = match secret_manager {
                    Some(secret_manager) => Some((&secret_manager).try_into()?),
                    None => None,
                };

                if let Some(secret_manager) = &secret_manager {
                    block_builder = block_builder.with_secret_manager(secret_manager);
                }

                if let Some(options) = options {
                    block_builder = block_builder.set_options(options).await?;
                }

                Ok(Response::PreparedTransactionData(PreparedTransactionDataDto::from(
                    &block_builder.prepare_transaction().await?,
                )))
            }
            Message::SignTransaction {
                secret_manager,
                prepared_transaction_data,
            } => {
                let mut block_builder = self.client.block();

                let secret_manager = (&secret_manager).try_into()?;

                block_builder = block_builder.with_secret_manager(&secret_manager);

                Ok(Response::SignedTransaction(PayloadDto::from(
                    &block_builder
                        .sign_transaction(PreparedTransactionData::try_from_dto_unverified(
                            &prepared_transaction_data,
                        )?)
                        .await?,
                )))
            }
            Message::SignatureUnlock {
                secret_manager,
                input_signing_data,
                transaction_essence_hash,
                remainder_data,
            } => {
                let token_supply: u64 = self.client.get_token_supply().await?;
                let secret_manager: SecretManager = (&secret_manager).try_into()?;
                let input_signing_data: InputSigningData =
                    InputSigningData::try_from_dto(&input_signing_data, token_supply)?;
                let transaction_essence_hash: [u8; 32] = transaction_essence_hash
                    .try_into()
                    .map_err(|_| DtoError::InvalidField("expected 32 bytes for transactionEssenceHash"))?;
                let remainder_data: Option<RemainderData> = remainder_data
                    .map(|remainder| RemainderData::try_from_dto(&remainder, token_supply))
                    .transpose()?;

                let unlock: Unlock = secret_manager
                    .signature_unlock(&input_signing_data, &transaction_essence_hash, &remainder_data)
                    .await?;

                Ok(Response::SignatureUnlock((&unlock).into()))
            }
            #[cfg(feature = "stronghold")]
            Message::StoreMnemonic {
                secret_manager,
                mnemonic,
            } => {
                let mut secret_manager = (&secret_manager).try_into()?;
                if let SecretManager::Stronghold(secret_manager) = &mut secret_manager {
                    secret_manager.store_mnemonic(mnemonic).await?;
                } else {
                    return Err(crate::Error::SecretManagerMismatch);
                }

                Ok(Response::Ok)
            }
            Message::PostBlockPayload { payload_dto } => {
                let block_builder = self.client.block();

                let block = block_builder
                    .finish_block(Some(Payload::try_from_dto(
                        &payload_dto,
                        &self.client.get_protocol_parameters().await?,
                    )?))
                    .await?;

                let block_id = block.id();

                Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
            }
            #[cfg(not(target_family = "wasm"))]
            Message::UnhealthyNodes => Ok(Response::UnhealthyNodes(
                self.client.unhealthy_nodes().into_iter().cloned().collect(),
            )),
            Message::GetHealth { url } => Ok(Response::Health(self.client.get_health(&url).await?)),
            Message::GetNodeInfo { url, auth } => Ok(Response::NodeInfo(Client::get_node_info(&url, auth).await?)),
            Message::GetInfo => Ok(Response::Info(self.client.get_info().await?)),
            Message::GetPeers => Ok(Response::Peers(self.client.get_peers().await?)),
            Message::GetTips => Ok(Response::Tips(self.client.get_tips().await?)),
            Message::PostBlockRaw { block_bytes } => Ok(Response::BlockId(
                self.client
                    .post_block_raw(&Block::unpack_strict(
                        &block_bytes[..],
                        &self.client.get_protocol_parameters().await?,
                    )?)
                    .await?,
            )),
            Message::PostBlock { block } => Ok(Response::BlockId(
                self.client
                    .post_block(&Block::try_from_dto(
                        &block,
                        &self.client.get_protocol_parameters().await?,
                    )?)
                    .await?,
            )),
            Message::GetBlock { block_id } => Ok(Response::Block(BlockDto::from(
                &self.client.get_block(&block_id).await?,
            ))),
            Message::GetBlockMetadata { block_id } => Ok(Response::BlockMetadata(
                self.client.get_block_metadata(&block_id).await?,
            )),
            Message::GetBlockRaw { block_id } => Ok(Response::BlockRaw(self.client.get_block_raw(&block_id).await?)),
            Message::GetOutput { output_id } => Ok(Response::Output(self.client.get_output(&output_id).await?)),
            Message::GetOutputMetadata { output_id } => Ok(Response::OutputMetadata(
                self.client.get_output_metadata(&output_id).await?,
            )),
            Message::GetMilestoneById { milestone_id } => Ok(Response::Milestone(MilestonePayloadDto::from(
                &self.client.get_milestone_by_id(&milestone_id).await?,
            ))),
            Message::GetMilestoneByIdRaw { milestone_id } => Ok(Response::MilestoneRaw(
                self.client.get_milestone_by_id_raw(&milestone_id).await?,
            )),
            Message::GetMilestoneByIndex { index } => Ok(Response::Milestone(MilestonePayloadDto::from(
                &self.client.get_milestone_by_index(index).await?,
            ))),
            Message::GetMilestoneByIndexRaw { index } => Ok(Response::MilestoneRaw(
                self.client.get_milestone_by_index_raw(index).await?,
            )),
            Message::GetUtxoChangesById { milestone_id } => Ok(Response::MilestoneUtxoChanges(
                self.client.get_utxo_changes_by_id(&milestone_id).await?,
            )),
            Message::GetUtxoChangesByIndex { index } => Ok(Response::MilestoneUtxoChanges(
                self.client.get_utxo_changes_by_index(index).await?,
            )),
            Message::GetReceipts => Ok(Response::Receipts(self.client.get_receipts().await?)),
            Message::GetReceiptsMigratedAt { milestone_index } => Ok(Response::Receipts(
                self.client.get_receipts_migrated_at(milestone_index).await?,
            )),
            Message::GetTreasury => Ok(Response::Treasury(self.client.get_treasury().await?)),
            Message::GetIncludedBlock { transaction_id } => Ok(Response::Block(BlockDto::from(
                &self.client.get_included_block(&transaction_id).await?,
            ))),
            Message::GetIncludedBlockMetadata { transaction_id } => Ok(Response::BlockMetadata(
                self.client.get_included_block_metadata(&transaction_id).await?,
            )),
            Message::BasicOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                self.client.basic_output_ids(query_parameters).await?,
            )),
            Message::AliasOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                self.client.alias_output_ids(query_parameters).await?,
            )),
            Message::AliasOutputId { alias_id } => Ok(Response::OutputId(self.client.alias_output_id(alias_id).await?)),
            Message::NftOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                self.client.nft_output_ids(query_parameters).await?,
            )),
            Message::NftOutputId { nft_id } => Ok(Response::OutputId(self.client.nft_output_id(nft_id).await?)),
            Message::FoundryOutputIds { query_parameters } => Ok(Response::OutputIdsResponse(
                self.client.foundry_output_ids(query_parameters).await?,
            )),
            Message::FoundryOutputId { foundry_id } => {
                Ok(Response::OutputId(self.client.foundry_output_id(foundry_id).await?))
            }
            Message::GetOutputs { output_ids } => Ok(Response::Outputs(self.client.get_outputs(output_ids).await?)),
            Message::TryGetOutputs { output_ids } => {
                Ok(Response::Outputs(self.client.try_get_outputs(output_ids).await?))
            }
            Message::FindBlocks { block_ids } => Ok(Response::Blocks(
                self.client
                    .find_blocks(&block_ids)
                    .await?
                    .iter()
                    .map(BlockDto::from)
                    .collect(),
            )),
            Message::Retry { block_id } => {
                let (block_id, block) = self.client.retry(&block_id).await?;
                Ok(Response::BlockIdWithBlock(block_id, BlockDto::from(&block)))
            }
            Message::RetryUntilIncluded {
                block_id,
                interval,
                max_attempts,
            } => {
                let res = self
                    .client
                    .retry_until_included(&block_id, interval, max_attempts)
                    .await?;
                let res = res
                    .into_iter()
                    .map(|(block_id, block)| (block_id, BlockDto::from(&block)))
                    .collect();
                Ok(Response::RetryUntilIncludedSuccessful(res))
            }
            Message::ConsolidateFunds {
                secret_manager,
                generate_addresses_options,
            } => {
                let secret_manager = (&secret_manager).try_into()?;
                Ok(Response::ConsolidatedFunds(
                    self.client
                        .consolidate_funds(&secret_manager, generate_addresses_options)
                        .await?,
                ))
            }
            Message::FindInputs { addresses, amount } => Ok(Response::Inputs(
                self.client
                    .find_inputs(addresses, amount)
                    .await?
                    .iter()
                    .map(UtxoInputDto::from)
                    .collect(),
            )),
            Message::FindOutputs { output_ids, addresses } => Ok(Response::Outputs(
                self.client.find_outputs(&output_ids, &addresses).await?,
            )),
            Message::Reattach { block_id } => {
                let (block_id, block) = self.client.reattach(&block_id).await?;
                Ok(Response::Reattached((block_id, BlockDto::from(&block))))
            }
            Message::ReattachUnchecked { block_id } => {
                let (block_id, block) = self.client.reattach_unchecked(&block_id).await?;
                Ok(Response::Reattached((block_id, BlockDto::from(&block))))
            }
            Message::Promote { block_id } => {
                let (block_id, block) = self.client.promote(&block_id).await?;
                Ok(Response::Promoted((block_id, BlockDto::from(&block))))
            }
            Message::PromoteUnchecked { block_id } => {
                let (block_id, block) = self.client.promote_unchecked(&block_id).await?;
                Ok(Response::Promoted((block_id, BlockDto::from(&block))))
            }
            Message::Bech32ToHex { bech32 } => Ok(Response::Bech32ToHex(Client::bech32_to_hex(&bech32)?)),
            Message::HexToBech32 { hex, bech32_hrp } => Ok(Response::Bech32Address(
                self.client.hex_to_bech32(&hex, bech32_hrp.as_deref()).await?,
            )),
            Message::AliasIdToBech32 { alias_id, bech32_hrp } => Ok(Response::Bech32Address(
                self.client.alias_id_to_bech32(alias_id, bech32_hrp.as_deref()).await?,
            )),
            Message::NftIdToBech32 { nft_id, bech32_hrp } => Ok(Response::Bech32Address(
                self.client.nft_id_to_bech32(nft_id, bech32_hrp.as_deref()).await?,
            )),
            Message::HexPublicKeyToBech32Address { hex, bech32_hrp } => Ok(Response::Bech32Address(
                self.client
                    .hex_public_key_to_bech32_address(&hex, bech32_hrp.as_deref())
                    .await?,
            )),
            Message::ParseBech32Address { address } => Ok(Response::ParsedBech32Address(AddressDto::from(
                &Client::parse_bech32_address(&address)?,
            ))),
            Message::IsAddressValid { address } => Ok(Response::IsAddressValid(Client::is_address_valid(&address))),
            Message::GenerateMnemonic => Ok(Response::GeneratedMnemonic(Client::generate_mnemonic()?)),
            Message::MnemonicToHexSeed { mut mnemonic } => {
                let response = Response::MnemonicHexSeed(Client::mnemonic_to_hex_seed(&mnemonic)?);

                mnemonic.zeroize();

                Ok(response)
            }
            Message::BlockId { block } => {
                let block = Block::try_from_dto_unverified(&block)?;
                Ok(Response::BlockId(block.id()))
            }
            Message::TransactionId { payload } => {
                let payload = TransactionPayload::try_from_dto_unverified(&payload)?;
                Ok(Response::TransactionId(payload.id()))
            }
            Message::ComputeAliasId { output_id } => Ok(Response::AliasId(AliasId::from(&output_id))),
            Message::ComputeNftId { output_id } => Ok(Response::NftId(NftId::from(&output_id))),
            Message::ComputeFoundryId {
                alias_address,
                serial_number,
                token_scheme_kind,
            } => Ok(Response::FoundryId(FoundryId::build(
                &alias_address,
                serial_number,
                token_scheme_kind,
            ))),
            Message::Faucet { url, address } => Ok(Response::Faucet(request_funds_from_faucet(&url, &address).await?)),
            Message::HashTransactionEssence { essence } => Ok(Response::TransactionEssenceHash(prefix_hex::encode(
                TransactionEssence::try_from_dto_unverified(&essence)?.hash(),
            ))),
        }
    }
}
