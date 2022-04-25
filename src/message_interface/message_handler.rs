// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{any::Any, panic::AssertUnwindSafe};

use backtrace::Backtrace;
use bee_message::{
    address::dto::AddressDto,
    input::dto::UtxoInputDto,
    payload::{dto::PayloadDto, Payload},
    Message as BeeMessage, MessageDto,
};
use futures::{Future, FutureExt};

use crate::{
    message_interface::{
        client_method::ClientMethod, message::Message, message_type::MessageType, response::Response,
        response_type::ResponseType,
    },
    signing::SignerHandle,
    Client, Result,
};

fn panic_to_response_message(panic: Box<dyn Any>) -> ResponseType {
    let msg = if let Some(message) = panic.downcast_ref::<String>() {
        format!("Internal error: {}", message)
    } else if let Some(message) = panic.downcast_ref::<&str>() {
        format!("Internal error: {}", message)
    } else {
        "Internal error".to_string()
    };
    let current_backtrace = Backtrace::new();
    ResponseType::Panic(format!("{}\n\n{:?}", msg, current_backtrace))
}

async fn convert_async_panics<F>(f: impl FnOnce() -> F) -> Result<ResponseType>
where
    F: Future<Output = Result<ResponseType>>,
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
        let response: Result<ResponseType> = match message.message_type_mut() {
            MessageType::CallClientMethod(method) => {
                convert_async_panics(|| async { self.call_client_method(method).await }).await
            }
        };

        let response = match response {
            Ok(r) => r,
            Err(e) => ResponseType::Error(e),
        };
        let _ = message.response_tx.send(Response::new(message.message_type, response));
    }

    async fn call_client_method(&self, method: &ClientMethod) -> Result<ResponseType> {
        match method {
            ClientMethod::GenerateAddresses { signer, options } => {
                let signer = SignerHandle::from_str(signer)?;
                let addresses = self
                    .client
                    .get_addresses(&signer)
                    .set_options(options.clone())?
                    .finish()
                    .await?;
                Ok(ResponseType::GeneratedAddresses(addresses))
            }
            ClientMethod::GenerateMessage { signer, options } => {
                let mut message_builder = self.client.message();

                let signer = match signer {
                    Some(signer) => Some(SignerHandle::from_str(signer)?),
                    None => None,
                };

                if let Some(signer) = &signer {
                    message_builder = message_builder.with_signer(signer);
                }

                if let Some(options) = options {
                    message_builder = message_builder.set_options(options.clone())?;
                }

                Ok(ResponseType::GeneratedMessage(MessageDto::from(
                    &message_builder.finish().await?,
                )))
            }
            ClientMethod::GetNode => Ok(ResponseType::Node(self.client.get_node().await?)),
            ClientMethod::GetNetworkInfo => Ok(ResponseType::NetworkInfo(self.client.get_network_info().await?)),
            ClientMethod::GetNetworkId => Ok(ResponseType::NetworkId(self.client.get_network_id().await?)),
            ClientMethod::GetBech32Hrp => Ok(ResponseType::Bech32Hrp(self.client.get_bech32_hrp().await?)),
            ClientMethod::GetMinPoWScore => Ok(ResponseType::MinPoWScore(self.client.get_min_pow_score().await?)),
            ClientMethod::GetTipsInterval => Ok(ResponseType::TipsInterval(self.client.get_tips_interval().await)),
            ClientMethod::GetLocalPoW => Ok(ResponseType::LocalPoW(self.client.get_local_pow().await)),
            ClientMethod::GetFallbackToLocalPoW => Ok(ResponseType::FallbackToLocalPoW(
                self.client.get_fallback_to_local_pow().await,
            )),
            ClientMethod::PrepareTransaction { signer, options } => {
                let mut message_builder = self.client.message();

                let signer = match signer {
                    Some(signer) => Some(SignerHandle::from_str(signer)?),
                    None => None,
                };

                if let Some(signer) = &signer {
                    message_builder = message_builder.with_signer(signer);
                }

                if let Some(options) = options {
                    message_builder = message_builder.set_options(options.clone())?;
                }

                Ok(ResponseType::PreparedTransactionData(
                    message_builder.prepare_transaction().await?,
                ))
            }
            ClientMethod::SignTransaction {
                signer,
                prepared_transaction_data,
            } => {
                let mut message_builder = self.client.message();

                let signer = SignerHandle::from_str(signer)?;

                message_builder = message_builder.with_signer(&signer);

                Ok(ResponseType::SignedTransaction(PayloadDto::from(
                    &message_builder
                        .sign_transaction(prepared_transaction_data.clone())
                        .await?,
                )))
            }
            ClientMethod::SubmitPayload { payload_dto } => {
                let message_builder = self.client.message();

                Ok(ResponseType::GeneratedMessage(MessageDto::from(
                    &message_builder
                        .finish_message(Some(Payload::try_from(payload_dto)?))
                        .await?,
                )))
            }
            #[cfg(not(feature = "wasm"))]
            ClientMethod::UnsyncedNodes => Ok(ResponseType::UnsyncedNodes(
                self.client.unsynced_nodes().await.into_iter().cloned().collect(),
            )),
            ClientMethod::GetNodeHealth { url } => Ok(ResponseType::NodeHealth(Client::get_node_health(url).await?)),
            ClientMethod::GetHealth => Ok(ResponseType::NodeHealth(self.client.get_health().await?)),
            ClientMethod::GetNodeInfo { url, auth } => {
                Ok(ResponseType::NodeInfo(Client::get_node_info(url, auth.clone()).await?))
            }
            ClientMethod::GetInfo => Ok(ResponseType::Info(self.client.get_info().await?)),
            ClientMethod::GetPeers => Ok(ResponseType::Peers(self.client.get_peers().await?)),
            ClientMethod::GetTips => Ok(ResponseType::Tips(self.client.get_tips().await?)),
            ClientMethod::PostMessage { message } => Ok(ResponseType::PostMessageSuccessful(
                self.client.post_message(&BeeMessage::try_from(message)?).await?,
            )),
            ClientMethod::PostMessageJson { message } => Ok(ResponseType::PostMessageSuccessful(
                self.client.post_message_json(&BeeMessage::try_from(message)?).await?,
            )),
            ClientMethod::GetMessageData { message_id } => Ok(ResponseType::MessageData(MessageDto::from(
                &self.client.get_message_data(message_id).await?,
            ))),
            ClientMethod::GetMessageMetadata { message_id } => Ok(ResponseType::MessageMetadata(
                self.client.get_message_metadata(message_id).await?,
            )),
            ClientMethod::GetMessageRaw { message_id } => {
                Ok(ResponseType::MessageRaw(self.client.get_message_raw(message_id).await?))
            }
            ClientMethod::GetMessageChildren { message_id } => Ok(ResponseType::MessageChildren(
                self.client.get_message_children(message_id).await?,
            )),
            ClientMethod::GetOutput { output_id } => Ok(ResponseType::Output(self.client.get_output(output_id).await?)),
            ClientMethod::GetMilestone { index } => {
                Ok(ResponseType::Milestone(self.client.get_milestone(*index).await?))
            }
            ClientMethod::GetMilestoneUtxoChanges { index } => Ok(ResponseType::MilestoneUtxoChanges(
                self.client.get_milestone_utxo_changes(*index).await?,
            )),
            ClientMethod::GetReceipts => Ok(ResponseType::Receipts(self.client.get_receipts().await?)),
            ClientMethod::GetReceiptsMigratedAt { milestone_index } => Ok(ResponseType::ReceiptsMigratedAtMilestone(
                self.client.get_receipts_migrated_at(*milestone_index).await?,
            )),
            ClientMethod::GetTreasury => Ok(ResponseType::Treasury(self.client.get_treasury().await?)),
            ClientMethod::GetIncludedMessage { transaction_id } => Ok(ResponseType::IncludedMessage(MessageDto::from(
                &self.client.get_included_message(transaction_id).await?,
            ))),
            ClientMethod::OutputIds { query_parameters } => Ok(ResponseType::OutputIds(
                self.client.output_ids(query_parameters.clone()).await?,
            )),
            ClientMethod::AliasesOutputIds { query_parameters } => Ok(ResponseType::OutputIds(
                self.client.aliases_output_ids(query_parameters.clone()).await?,
            )),
            ClientMethod::AliasOutputId { alias_id } => {
                Ok(ResponseType::OutputId(self.client.alias_output_id(*alias_id).await?))
            }
            ClientMethod::NftsOutputIds { query_parameters } => Ok(ResponseType::OutputIds(
                self.client.nfts_output_ids(query_parameters.clone()).await?,
            )),
            ClientMethod::NftOutputId { nft_id } => {
                Ok(ResponseType::OutputId(self.client.nft_output_id(*nft_id).await?))
            }
            ClientMethod::FoundriesOutputIds { query_parameters } => Ok(ResponseType::OutputIds(
                self.client.foundries_output_ids(query_parameters.clone()).await?,
            )),
            ClientMethod::FoundryOutputId { foundry_id } => Ok(ResponseType::OutputId(
                self.client.foundry_output_id(*foundry_id).await?,
            )),
            ClientMethod::GetOutputs { output_ids } => Ok(ResponseType::Outputs(
                self.client.get_outputs(output_ids.clone()).await?,
            )),
            ClientMethod::TryGetOutputs { output_ids } => Ok(ResponseType::Outputs(
                self.client.try_get_outputs(output_ids.clone()).await?,
            )),
            ClientMethod::FindMessages { message_ids } => Ok(ResponseType::Messages(
                self.client
                    .find_messages(message_ids)
                    .await?
                    .iter()
                    .map(MessageDto::from)
                    .collect(),
            )),
            ClientMethod::Retry { message_id } => {
                let (message_id, message) = self.client.retry(message_id).await?;
                Ok(ResponseType::RetrySuccessful((message_id, MessageDto::from(&message))))
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
                Ok(ResponseType::RetryUntilIncludedSuccessful(res))
            }
            ClientMethod::ConsolidateFunds {
                signer,
                account_index,
                address_range,
            } => {
                let signer = SignerHandle::from_str(signer)?;
                Ok(ResponseType::ConsolidatedFunds(
                    self.client
                        .consolidate_funds(&signer, *account_index, address_range.clone())
                        .await?,
                ))
            }
            ClientMethod::FindInputs { addresses, amount } => Ok(ResponseType::Inputs(
                self.client
                    .find_inputs(addresses.clone(), *amount)
                    .await?
                    .iter()
                    .map(UtxoInputDto::from)
                    .collect(),
            )),
            ClientMethod::FindOutputs { outputs, addresses } => Ok(ResponseType::Outputs(
                self.client.find_outputs(outputs, addresses).await?,
            )),
            ClientMethod::Reattach { message_id } => {
                let (message_id, message) = self.client.reattach(message_id).await?;
                Ok(ResponseType::Reattached((message_id, MessageDto::from(&message))))
            }
            ClientMethod::ReattachUnchecked { message_id } => {
                let (message_id, message) = self.client.reattach_unchecked(message_id).await?;
                Ok(ResponseType::Reattached((message_id, MessageDto::from(&message))))
            }
            ClientMethod::Promote { message_id } => {
                let (message_id, message) = self.client.promote(message_id).await?;
                Ok(ResponseType::Promoted((message_id, MessageDto::from(&message))))
            }
            ClientMethod::PromoteUnchecked { message_id } => {
                let (message_id, message) = self.client.promote_unchecked(message_id).await?;
                Ok(ResponseType::Promoted((message_id, MessageDto::from(&message))))
            }
            ClientMethod::Bech32ToHex { bech32 } => Ok(ResponseType::Bech32ToHex(Client::bech32_to_hex(bech32)?)),
            ClientMethod::HexToBech32 { hex, bech32_hrp } => Ok(ResponseType::HexToBech32(
                self.client.hex_to_bech32(hex, bech32_hrp.as_deref()).await?,
            )),
            ClientMethod::HexPublicKeyToBech32Address { hex, bech32_hrp } => Ok(ResponseType::HexToBech32(
                self.client
                    .hex_public_key_to_bech32_address(hex, bech32_hrp.as_deref())
                    .await?,
            )),
            ClientMethod::ParseBech32Address { address } => Ok(ResponseType::ParsedBech32Address(AddressDto::from(
                &Client::parse_bech32_address(address)?,
            ))),
            ClientMethod::IsAddressValid { address } => {
                Ok(ResponseType::IsAddressValid(Client::is_address_valid(address)))
            }
            ClientMethod::GenerateMnemonic => Ok(ResponseType::GeneratedMnemonic(Client::generate_mnemonic()?)),
            ClientMethod::MnemonicToHexSeed { mnemonic } => {
                Ok(ResponseType::MnemonicHexSeed(Client::mnemonic_to_hex_seed(mnemonic)?))
            }
        }
    }
}
