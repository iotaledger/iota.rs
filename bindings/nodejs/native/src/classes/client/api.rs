// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{convert::TryInto, ops::Range, str::FromStr};

use super::MessageDto;

use crate::classes::client::dto::{AddressBalanceDto, MessageWrapper, OutputMetadataDto};
use iota_client::{
    bee_message::prelude::{Address, MessageBuilder, MessageId, Parents, TransactionId, UtxoInput},
    bee_rest_api::types::dtos::{AddressDto, MessageDto as BeeMessageDto, OutputDto as BeeOutput},
    AddressOutputsOptions, ClientMiner, Seed,
};
use neon::prelude::*;

pub(crate) enum Api {
    // High level APIs
    Send {
        seed: Option<Seed>,
        index: Option<Vec<u8>>,
        data: Option<Vec<u8>>,
        parents: Option<Vec<MessageId>>,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
        inputs: Vec<UtxoInput>,
        input_range: Option<Range<usize>>,
        outputs: Vec<(Address, u64)>,
        dust_allowance_outputs: Vec<(Address, u64)>,
    },
    GetUnspentAddress {
        seed: Seed,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
    },
    GetAddresses {
        seed: Seed,
        account_index: Option<usize>,
        range: Option<Range<usize>>,
        bech32_hrp: Option<String>,
        include_internal: bool,
    },
    FindMessages {
        indexation_keys: Vec<String>,
        message_ids: Vec<MessageId>,
    },
    GetBalance {
        seed: Seed,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
        gap_limit: Option<usize>,
    },
    GetAddressBalances(Vec<String>),
    // Node APIs
    GetInfo,
    GetNetworkInfo,
    GetPeers,
    GetTips,
    PostMessage(MessageDto),
    GetMessagesByIndexation(Vec<u8>),
    GetMessage(MessageId),
    GetMessageMetadata(MessageId),
    GetRawMessage(MessageId),
    GetMessageChildren(MessageId),
    GetOutput(UtxoInput),
    FindOutputs {
        outputs: Vec<UtxoInput>,
        addresses: Vec<String>,
    },
    GetAddressBalance(String),
    GetAddressOutputs(String, AddressOutputsOptions),
    GetMilestone(u32),
    GetMilestoneUtxoChanges(u32),
    GetReceipts(),
    GetReceiptsMigratedAt(u32),
    GetTreasury(),
    GetIncludedMessage(TransactionId),
    Retry(MessageId),
    RetryUntilIncluded(MessageId, Option<u64>, Option<u64>),
    ConsolidateFunds(Seed, usize, usize, usize),
    Reattach(MessageId),
    Promote(MessageId),
    HexToBech32(String, Option<String>),
}

pub(crate) struct ClientTask {
    pub client_id: String,
    pub api: Api,
}

impl Task for ClientTask {
    type Output = String;
    type Error = crate::Error;
    type JsEvent = JsString;
    // TODO: Try async-mutex
    #[allow(clippy::await_holding_lock)]
    fn perform(&self) -> Result<Self::Output, Self::Error> {
        crate::block_on(crate::convert_async_panics(|| async move {
            let client = crate::get_client(&self.client_id);
            let client = client.read().expect("Failed to read client");
            let res = match &self.api {
                // High level API
                Api::Send {
                    seed,
                    index,
                    data,
                    parents,
                    account_index,
                    initial_address_index,
                    inputs,
                    input_range,
                    outputs,
                    dust_allowance_outputs,
                } => {
                    let mut sender = client.message();
                    if let Some(seed) = seed {
                        sender = sender.with_seed(seed);
                    }
                    if let Some(index) = index {
                        sender = sender.with_index(index);
                    }
                    if let Some(data) = data {
                        sender = sender.with_data(data.clone());
                    }
                    if let Some(parents) = parents {
                        sender = sender.with_parents(parents.clone())?;
                    }
                    if let Some(account_index) = account_index {
                        sender = sender.with_account_index(*account_index);
                    }
                    if let Some(initial_address_index) = initial_address_index {
                        sender = sender.with_initial_address_index(*initial_address_index);
                    }
                    for input in inputs {
                        sender = sender.with_input(input.clone());
                    }
                    if let Some(input_range) = input_range {
                        sender = sender.with_input_range(input_range.clone());
                    }
                    let bech32_hrp = client.get_bech32_hrp().await?;
                    for output in outputs {
                        sender = sender.with_output(&output.0.clone().to_bech32(&bech32_hrp), output.1)?;
                    }
                    for output in dust_allowance_outputs {
                        sender =
                            sender.with_dust_allowance_output(&output.0.clone().to_bech32(&bech32_hrp), output.1)?;
                    }
                    let message = sender.finish().await?;
                    serde_json::to_string(&MessageWrapper {
                        message_id: message.id().0,
                        message: BeeMessageDto::from(&message),
                    })?
                }
                Api::GetUnspentAddress {
                    seed,
                    account_index,
                    initial_address_index,
                } => {
                    let mut getter = client.get_unspent_address(seed);
                    if let Some(account_index) = account_index {
                        getter = getter.with_account_index(*account_index);
                    }
                    if let Some(initial_address_index) = initial_address_index {
                        getter = getter.with_initial_address_index(*initial_address_index);
                    }
                    let (address, index) = getter.get().await?;
                    serde_json::to_string(&(address, index))?
                }
                Api::GetAddresses {
                    seed,
                    account_index,
                    range,
                    bech32_hrp,
                    include_internal,
                } => {
                    let mut getter = client.get_addresses(&seed);
                    if let Some(account_index) = account_index {
                        getter = getter.with_account_index(*account_index);
                    }
                    if let Some(range) = range {
                        getter = getter.with_range(range.clone());
                    }

                    if let Some(bech32_hrp) = bech32_hrp {
                        getter = getter.with_bech32_hrp(bech32_hrp.clone())
                    }

                    if *include_internal {
                        let all_addresses = getter.get_all().await?;
                        return Ok(serde_json::to_string(&all_addresses)?);
                    }
                    let public_addresses = getter.finish().await?;
                    serde_json::to_string(&public_addresses)?
                }
                Api::FindMessages {
                    indexation_keys,
                    message_ids,
                } => {
                    let messages = client.find_messages(&indexation_keys[..], &message_ids[..]).await?;
                    let message_wrappers: Vec<MessageWrapper> = messages
                        .into_iter()
                        .map(|message| MessageWrapper {
                            message_id: message.id().0,
                            message: BeeMessageDto::from(&message),
                        })
                        .collect();
                    serde_json::to_string(&message_wrappers)?
                }
                Api::GetBalance {
                    seed,
                    account_index,
                    initial_address_index,
                    gap_limit,
                } => {
                    let mut getter = client.get_balance(seed);
                    if let Some(account_index) = account_index {
                        getter = getter.with_account_index(*account_index);
                    }
                    if let Some(initial_address_index) = initial_address_index {
                        getter = getter.with_initial_address_index(*initial_address_index);
                    }
                    if let Some(gap_limit) = gap_limit {
                        getter = getter.with_gap_limit(*gap_limit);
                    }
                    let balance = getter.finish().await?;
                    serde_json::to_string(&balance)?
                }
                Api::GetAddressBalances(bech32_addresses) => {
                    let balances = client.get_address_balances(&bech32_addresses[..]).await?;
                    let mut bech32_balances = Vec::new();
                    for balance in balances {
                        bech32_balances.push(AddressBalanceDto {
                            address: client.hex_to_bech32(&balance.address.to_string(), None).await?,
                            balance: balance.balance,
                            dust_allowed: balance.dust_allowed,
                        })
                    }
                    serde_json::to_string(&bech32_balances)?
                }
                // Node APIs
                Api::GetInfo => serde_json::to_string(&client.get_info().await?)?,
                Api::GetNetworkInfo => serde_json::to_string(&client.get_network_info().await?)?,
                Api::GetPeers => serde_json::to_string(&client.get_peers().await?)?,
                Api::GetTips => {
                    let tips = client.get_tips().await?;
                    serde_json::to_string(&tips)?
                }
                Api::PostMessage(message) => {
                    let parent_msg_ids = match message.parents.as_ref() {
                        Some(parents) => {
                            let mut parent_ids = Vec::new();
                            for msg_id in parents {
                                parent_ids.push(MessageId::from_str(&msg_id)?)
                            }
                            parent_ids
                        }
                        None => client.get_tips().await?,
                    };
                    let network_id = client.get_network_id().await?;
                    let nonce_provider = client.get_pow_provider().await;
                    let message = MessageBuilder::<ClientMiner>::new()
                        .with_network_id(network_id)
                        .with_parents(Parents::new(parent_msg_ids)?)
                        .with_nonce_provider(nonce_provider, 4000f64)
                        .with_payload(message.payload.clone().try_into()?)
                        .finish()?;
                    let message = client.post_message(&message).await?;
                    serde_json::to_string(&message)?
                }
                Api::GetMessagesByIndexation(index) => {
                    let messages = client.get_message().index(index).await?;
                    serde_json::to_string(&messages)?
                }
                Api::GetMessage(id) => {
                    let message = client.get_message().data(&id).await?;
                    serde_json::to_string(&MessageWrapper {
                        message_id: message.id().0,
                        message: BeeMessageDto::from(&message),
                    })?
                }
                Api::GetMessageMetadata(id) => {
                    let metadata = client.get_message().metadata(&id).await?;
                    serde_json::to_string(&metadata)?
                }
                Api::GetRawMessage(id) => client.get_message().raw(&id).await?,
                Api::GetMessageChildren(id) => {
                    let messages = client.get_message().children(&id).await?;
                    serde_json::to_string(&messages)?
                }
                Api::GetOutput(id) => {
                    let output = client.get_output(id).await?;
                    let (output_amount, output_address) = match output.output {
                        BeeOutput::Treasury(t) => (t.amount, "".to_string()),
                        BeeOutput::SignatureLockedSingle(r) => match r.address {
                            AddressDto::Ed25519(addr) => (r.amount, addr.address),
                        },
                        BeeOutput::SignatureLockedDustAllowance(r) => match r.address {
                            AddressDto::Ed25519(addr) => (r.amount, addr.address),
                        },
                    };
                    serde_json::to_string(&OutputMetadataDto {
                        message_id: output.message_id,
                        transaction_id: output.transaction_id,
                        output_index: output.output_index,
                        is_spent: output.is_spent,
                        address: client.hex_to_bech32(&output_address.to_string(), None).await?,
                        amount: output_amount,
                    })?
                }
                Api::FindOutputs { outputs, addresses } => {
                    let outputs = client.find_outputs(outputs, &addresses[..]).await?;
                    let mut bech32_outputs = Vec::new();
                    for output in outputs {
                        let (output_amount, output_address) = match output.output {
                            BeeOutput::Treasury(t) => (t.amount, "".to_string()),
                            BeeOutput::SignatureLockedSingle(r) => match r.address {
                                AddressDto::Ed25519(addr) => (r.amount, addr.address),
                            },
                            BeeOutput::SignatureLockedDustAllowance(r) => match r.address {
                                AddressDto::Ed25519(addr) => (r.amount, addr.address),
                            },
                        };
                        bech32_outputs.push(OutputMetadataDto {
                            message_id: output.message_id,
                            transaction_id: output.transaction_id,
                            output_index: output.output_index,
                            is_spent: output.is_spent,
                            address: client.hex_to_bech32(&output_address.to_string(), None).await?,
                            amount: output_amount,
                        })
                    }
                    serde_json::to_string(&bech32_outputs)?
                }
                Api::GetAddressBalance(address) => {
                    let balance = client.get_address().balance(address).await?;
                    serde_json::to_string(&AddressBalanceDto {
                        address: client.hex_to_bech32(&balance.address.to_string(), None).await?,
                        balance: balance.balance,
                        dust_allowed: balance.dust_allowed,
                    })?
                }
                Api::GetAddressOutputs(address, options) => {
                    let output_ids = client.get_address().outputs(address, options.clone()).await?;
                    serde_json::to_string(&output_ids)?
                }
                Api::GetMilestone(index) => {
                    let milestone = client.get_milestone(*index).await?;
                    serde_json::to_string(&milestone)?
                }
                Api::GetMilestoneUtxoChanges(index) => {
                    let milestone_utxo_changes = client.get_milestone_utxo_changes(*index).await?;
                    serde_json::to_string(&milestone_utxo_changes)?
                }
                Api::GetReceipts() => {
                    let receipts = client.get_receipts().await?;
                    serde_json::to_string(&receipts)?
                }
                Api::GetReceiptsMigratedAt(index) => {
                    let receipts = client.get_receipts_migrated_at(*index).await?;
                    serde_json::to_string(&receipts)?
                }
                Api::GetTreasury() => {
                    let treasury = client.get_treasury().await?;
                    serde_json::to_string(&treasury)?
                }
                Api::GetIncludedMessage(transaction_id) => {
                    let message = client.get_included_message(&*transaction_id).await?;
                    serde_json::to_string(&MessageWrapper {
                        message_id: message.id().0,
                        message: BeeMessageDto::from(&message),
                    })?
                }
                Api::Retry(message_id) => {
                    let message = client.retry(message_id).await?;
                    serde_json::to_string(&MessageWrapper {
                        message_id: message.0,
                        message: BeeMessageDto::from(&message.1),
                    })?
                }
                Api::RetryUntilIncluded(message_id, interval, max_attempts) => {
                    let messages = client
                        .retry_until_included(message_id, *interval, *max_attempts)
                        .await?;
                    messages
                        .into_iter()
                        .map(|msg| {
                            serde_json::to_string(&MessageWrapper {
                                message_id: msg.0,
                                message: BeeMessageDto::from(&msg.1),
                            })
                        })
                        .collect::<Result<String, serde_json::Error>>()?
                }
                Api::ConsolidateFunds(seed, account_index, start_index, end_index) => {
                    let address = client
                        .consolidate_funds(seed, *account_index, *start_index..*end_index)
                        .await?;
                    serde_json::to_string(&address)?
                }
                Api::Reattach(message_id) => {
                    let message = client.reattach(message_id).await?;
                    serde_json::to_string(&MessageWrapper {
                        message: BeeMessageDto::from(&message.1),
                        message_id: message.0,
                    })?
                }
                Api::Promote(message_id) => {
                    let message = client.promote(message_id).await?;
                    serde_json::to_string(&MessageWrapper {
                        message: BeeMessageDto::from(&message.1),
                        message_id: message.0,
                    })?
                }
                Api::HexToBech32(hex, bech32_hrp) => {
                    let opt = bech32_hrp.as_ref().map(|opt| opt.as_str());
                    let bech32 = client.hex_to_bech32(hex, opt).await?;
                    serde_json::to_string(&bech32)?
                }
            };
            Ok(res)
        }))
    }

    fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        match result {
            Ok(s) => Ok(cx.string(s)),
            Err(e) => cx.throw_error(format!("ClientTask error: {:?}", e)),
        }
    }
}
