// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{convert::TryInto, ops::Range, str::FromStr};

use super::MessageDto;

use crate::classes::client::dto::MessageWrapper;
use iota::{Address, Bech32Address, ClientMiner, MessageBuilder, MessageId, Seed, UTXOInput};
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
        inputs: Vec<UTXOInput>,
        outputs: Vec<(Address, u64)>,
        dust_allowance_outputs: Vec<(Address, u64)>,
    },
    GetUnspentAddress {
        seed: Seed,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
    },
    FindAddresses {
        seed: Seed,
        account_index: Option<usize>,
        range: Option<Range<usize>>,
        bech32_hrp: Option<String>,
    },
    FindMessages {
        indexation_keys: Vec<String>,
        message_ids: Vec<MessageId>,
    },
    GetBalance {
        seed: Seed,
        account_index: Option<usize>,
        initial_address_index: Option<usize>,
    },
    GetAddressBalances(Vec<Bech32Address>),
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
    GetOutput(UTXOInput),
    FindOutputs {
        outputs: Vec<UTXOInput>,
        addresses: Vec<Bech32Address>,
    },
    GetAddressBalance(Bech32Address),
    GetAddressOutputs(Bech32Address),
    GetMilestone(u32),
    GetMilestoneUTXOChanges(u32),
    Retry(MessageId),
    Reattach(MessageId),
    Promote(MessageId),
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
            let client = client.read().unwrap();
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
                    let bech32_hrp = client.get_bech32_hrp().await?;
                    for output in outputs {
                        sender = sender.with_output(&output.0.clone().to_bech32(&bech32_hrp).into(), output.1)?;
                    }
                    for output in dust_allowance_outputs {
                        sender = sender
                            .with_dust_allowance_output(&output.0.clone().to_bech32(&bech32_hrp).into(), output.1)?;
                    }
                    let message = sender.finish().await?;
                    serde_json::to_string(&MessageWrapper {
                        message_id: message.id().0,
                        message,
                    })
                    .unwrap()
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
                    serde_json::to_string(&(address, index)).unwrap()
                }
                Api::FindAddresses {
                    seed,
                    account_index,
                    range,
                    bech32_hrp,
                } => {
                    let mut getter = client.find_addresses(&seed);
                    if let Some(account_index) = account_index {
                        getter = getter.with_account_index(*account_index);
                    }
                    if let Some(range) = range {
                        getter = getter.with_range(range.clone());
                    }

                    if let Some(bech32_hrp) = bech32_hrp {
                        getter = getter.with_bech32_hrp(bech32_hrp.clone())
                    }

                    let addresses = getter.finish().await?;
                    serde_json::to_string(&addresses).unwrap()
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
                            message,
                        })
                        .collect();
                    serde_json::to_string(&message_wrappers).unwrap()
                }
                Api::GetBalance {
                    seed,
                    account_index,
                    initial_address_index,
                } => {
                    let mut getter = client.get_balance(seed);
                    if let Some(account_index) = account_index {
                        getter = getter.with_account_index(*account_index);
                    }
                    if let Some(initial_address_index) = initial_address_index {
                        getter = getter.with_initial_address_index(*initial_address_index);
                    }
                    let balance = getter.finish().await?;
                    serde_json::to_string(&balance).unwrap()
                }
                Api::GetAddressBalances(bech32_addresses) => {
                    let balances = client.get_address_balances(&bech32_addresses[..]).await?;
                    let balances: Vec<super::AddressBalanceDto> = balances.into_iter().map(|b| b.into()).collect();
                    serde_json::to_string(&balances).unwrap()
                }
                // Node APIs
                Api::GetInfo => serde_json::to_string(&client.get_info().await?).unwrap(),
                Api::GetNetworkInfo => serde_json::to_string(&client.get_network_info().await?).unwrap(),
                Api::GetPeers => serde_json::to_string(&client.get_peers().await?).unwrap(),
                Api::GetTips => {
                    let tips = client.get_tips().await?;
                    serde_json::to_string(&tips).unwrap()
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
                    let message = MessageBuilder::<ClientMiner>::new()
                        .with_network_id(client.get_network_id().await?)
                        .with_parents(parent_msg_ids)
                        .with_nonce_provider(client.get_pow_provider(), 4000f64, None)
                        .with_payload(message.payload.clone().try_into()?)
                        .finish()?;
                    let message = client.post_message(&message).await?;
                    serde_json::to_string(&message).unwrap()
                }
                Api::GetMessagesByIndexation(index) => {
                    let messages = client.get_message().index(index).await?;
                    serde_json::to_string(&messages).unwrap()
                }
                Api::GetMessage(id) => {
                    let message = client.get_message().data(&id).await?;
                    serde_json::to_string(&MessageWrapper {
                        message_id: message.id().0,
                        message,
                    })
                    .unwrap()
                }
                Api::GetMessageMetadata(id) => {
                    let metadata = client.get_message().metadata(&id).await?;
                    serde_json::to_string(&metadata).unwrap()
                }
                Api::GetRawMessage(id) => client.get_message().raw(&id).await?,
                Api::GetMessageChildren(id) => {
                    let messages = client.get_message().children(&id).await?;
                    serde_json::to_string(&messages).unwrap()
                }
                Api::GetOutput(id) => {
                    let output = client.get_output(id).await?;
                    let output: super::OutputMetadataDto = output.into();
                    serde_json::to_string(&output).unwrap()
                }
                Api::FindOutputs { outputs, addresses } => {
                    let outputs = client.find_outputs(outputs, &addresses[..]).await?;
                    let outputs: Vec<super::OutputMetadataDto> = outputs.into_iter().map(|o| o.into()).collect();
                    serde_json::to_string(&outputs).unwrap()
                }
                Api::GetAddressBalance(address) => {
                    let balance = client.get_address().balance(address).await?;
                    serde_json::to_string(&balance).unwrap()
                }
                Api::GetAddressOutputs(address) => {
                    let output_ids = client.get_address().outputs(address).await?;
                    serde_json::to_string(&output_ids).unwrap()
                }
                Api::GetMilestone(index) => {
                    let milestone = client.get_milestone(*index).await?;
                    serde_json::to_string(&milestone).unwrap()
                }
                Api::GetMilestoneUTXOChanges(index) => {
                    let milestone_utxo_changes = client.get_milestone_utxo_changes(*index).await?;
                    serde_json::to_string(&milestone_utxo_changes).unwrap()
                }
                Api::Retry(message_id) => {
                    let message = client.retry(message_id).await?;
                    serde_json::to_string(&MessageWrapper {
                        message: message.1,
                        message_id: message.0,
                    })
                    .unwrap()
                }
                Api::Reattach(message_id) => {
                    let message = client.reattach(message_id).await?;
                    serde_json::to_string(&MessageWrapper {
                        message: message.1,
                        message_id: message.0,
                    })
                    .unwrap()
                }
                Api::Promote(message_id) => {
                    let message = client.promote(message_id).await?;
                    serde_json::to_string(&MessageWrapper {
                        message: message.1,
                        message_id: message.0,
                    })
                    .unwrap()
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
