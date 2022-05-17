// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{any::Any, panic::AssertUnwindSafe};

use backtrace::Backtrace;
use bee_message::{
    address::dto::AddressDto,
    input::dto::UtxoInputDto,
    payload::{
        dto::{MilestonePayloadDto, PayloadDto},
        Payload,
    },
    Message as BeeMessage, MessageDto,
};
use futures::{Future, FutureExt};
use zeroize::Zeroize;

use crate::{
    api::{PreparedTransactionData, PreparedTransactionDataDto},
    message_interface::{client_method::ClientMethod, message::Message, message_type::MessageType, response::Response},
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
    pub async fn handle(&self, mut message: Message) {
        let result: Result<Response> = match message.message_type() {
            MessageType::CallClientMethod(method) => {
                convert_async_panics(|| async { self.call_client_method(method).await }).await
            }
        };

        // Zeroize secrets as soon as their missions are finished.
        match &mut message.message_type {
            #[cfg(feature = "stronghold")]
            MessageType::CallClientMethod(ClientMethod::StoreMnemonic { mnemonic, .. }) => mnemonic.zeroize(),
            MessageType::CallClientMethod(ClientMethod::MnemonicToHexSeed { mnemonic }) => mnemonic.zeroize(),

            // SecretManagerDto impl ZeroizeOnDrop, so we don't have to call zeroize() here.
            _ => (),
        };

        let response = match result {
            Ok(r) => r,
            Err(e) => Response::Error(e),
        };

        let _ = message.response_tx.send(response);
    }

    // If cfg(not(feature = "stronghold")) then secret_manager doesn't necessarily to be mutable, but otherwise it has
    // to be. Instead of rendering the code messy just because of this, we just allow unused mutable variables.
    #[allow(unused_mut)]
    async fn call_client_method(&self, method: &ClientMethod) -> Result<Response> {
        match method {
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
            ClientMethod::GenerateMessage {
                secret_manager,
                options,
            } => {
                // Prepare transaction
                let mut transaction_builder = self.client.message();

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

                Ok(Response::GeneratedMessage(MessageDto::from(
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
                let mut message_builder = self.client.message();

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
                    message_builder = message_builder.with_secret_manager(secret_manager);
                }

                if let Some(options) = options {
                    message_builder = message_builder.set_options(options.clone())?;
                }

                Ok(Response::PreparedTransactionData(PreparedTransactionDataDto::from(
                    &message_builder.prepare_transaction().await?,
                )))
            }
            ClientMethod::SignTransaction {
                secret_manager,
                prepared_transaction_data,
            } => {
                let mut message_builder = self.client.message();

                let mut secret_manager: SecretManager = secret_manager.try_into()?;
                // If we use stronghold we need to read the snapshot in case it hasn't been done already
                #[cfg(feature = "stronghold")]
                if let SecretManager::Stronghold(stronghold_secret_manager) = &mut secret_manager {
                    stronghold_secret_manager.read_stronghold_snapshot().await?;
                }

                message_builder = message_builder.with_secret_manager(&secret_manager);

                Ok(Response::SignedTransaction(PayloadDto::from(
                    &message_builder
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
                let message_builder = self.client.message();

                Ok(Response::GeneratedMessage(MessageDto::from(
                    &message_builder
                        .finish_message(Some(Payload::try_from(payload_dto)?))
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
            ClientMethod::PostMessageRaw { message } => Ok(Response::PostMessageSuccessful(
                self.client.post_message_raw(&BeeMessage::try_from(message)?).await?,
            )),
            ClientMethod::PostMessage { message } => Ok(Response::PostMessageSuccessful(
                self.client.post_message(&BeeMessage::try_from(message)?).await?,
            )),
            ClientMethod::GetMessage { message_id } => Ok(Response::Message(MessageDto::from(
                &self.client.get_message(message_id).await?,
            ))),
            ClientMethod::GetMessageMetadata { message_id } => Ok(Response::MessageMetadata(
                self.client.get_message_metadata(message_id).await?,
            )),
            ClientMethod::GetMessageRaw { message_id } => {
                Ok(Response::MessageRaw(self.client.get_message_raw(message_id).await?))
            }
            ClientMethod::GetMessageChildren { message_id } => Ok(Response::MessageChildren(
                self.client.get_message_children(message_id).await?,
            )),
            ClientMethod::GetOutput { output_id } => Ok(Response::Output(self.client.get_output(output_id).await?)),
            ClientMethod::GetMilestoneById { milestone_id } => Ok(Response::Milestone(MilestonePayloadDto::from(
                &self.client.get_milestone_by_id(milestone_id).await?,
            ))),
            ClientMethod::GetMilestoneByIdRaw { milestone_id } => Ok(Response::MilestoneRaw(
                &self.client.get_milestone_by_id_raw(milestone_id).await?,
            )),
            ClientMethod::GetMilestoneByIndex { index } => Ok(Response::Milestone(MilestonePayloadDto::from(
                &self.client.get_milestone_by_index(*index).await?,
            ))),
            ClientMethod::GetMilestoneByIndexRaw { index } => Ok(Response::MilestoneRaw(
                &self.client.get_milestone_by_index_raw(*index).await?,
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
            ClientMethod::GetIncludedMessage { transaction_id } => Ok(Response::IncludedMessage(MessageDto::from(
                &self.client.get_included_message(transaction_id).await?,
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
            ClientMethod::FindMessages { message_ids } => Ok(Response::Messages(
                self.client
                    .find_messages(message_ids)
                    .await?
                    .iter()
                    .map(MessageDto::from)
                    .collect(),
            )),
            ClientMethod::Retry { message_id } => {
                let (message_id, message) = self.client.retry(message_id).await?;
                Ok(Response::RetrySuccessful((message_id, MessageDto::from(&message))))
            }
            ClientMethod::RetryUntilIncluded {
                message_id,
                interval,
                max_attempts,
            } => {
                let res = self
                    .client
                    .retry_until_included(message_id, *interval, *max_attempts)
                    .await?;
                let res = res
                    .into_iter()
                    .map(|(message_id, message)| (message_id, MessageDto::from(&message)))
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
            ClientMethod::Reattach { message_id } => {
                let (message_id, message) = self.client.reattach(message_id).await?;
                Ok(Response::Reattached((message_id, MessageDto::from(&message))))
            }
            ClientMethod::ReattachUnchecked { message_id } => {
                let (message_id, message) = self.client.reattach_unchecked(message_id).await?;
                Ok(Response::Reattached((message_id, MessageDto::from(&message))))
            }
            ClientMethod::Promote { message_id } => {
                let (message_id, message) = self.client.promote(message_id).await?;
                Ok(Response::Promoted((message_id, MessageDto::from(&message))))
            }
            ClientMethod::PromoteUnchecked { message_id } => {
                let (message_id, message) = self.client.promote_unchecked(message_id).await?;
                Ok(Response::Promoted((message_id, MessageDto::from(&message))))
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
            ClientMethod::MessageId { message } => {
                let message = BeeMessage::try_from(message)?;

                Ok(Response::MessageId(message.id()))
            }
        }
    }
}
