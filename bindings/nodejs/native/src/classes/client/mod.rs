// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::unnecessary_wraps)]
use iota_client::{
    bee_message::prelude::{Address, MessageId, TransactionId, UtxoInput},
    AddressOutputsOptions, Client, OutputType, Seed,
};
use neon::prelude::*;
use serde::Deserialize;

use std::str::FromStr;

mod builder;
pub use builder::*;

mod dto;
use dto::*;
mod api;
use api::{Api, ClientTask};

mod message_getter;
pub use message_getter::JsMessageGetter;

mod message_sender;
pub use message_sender::JsMessageSender;

mod unspent_address_getter;
pub use unspent_address_getter::JsUnspentAddressGetter;

mod address_getter;
pub use address_getter::JsAddressGetter;

mod balance_getter;
pub use balance_getter::JsBalanceGetter;

/// Parses a bech32 address string.
fn parse_address(address: &str) -> crate::Result<Address> {
    Ok(Address::try_from_bech32(address).map_err(|_| anyhow::anyhow!("invalid address"))?)
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum OutputTypeDto {
    SignatureLockedSingle,
    SignatureLockedDustAllowance,
}

impl From<OutputTypeDto> for OutputType {
    fn from(value: OutputTypeDto) -> Self {
        match value {
            OutputTypeDto::SignatureLockedSingle => OutputType::SignatureLockedSingle,
            OutputTypeDto::SignatureLockedDustAllowance => OutputType::SignatureLockedSingle,
        }
    }
}

#[derive(Default, Deserialize)]
pub struct AddressOutputsOptionsDto {
    #[serde(rename = "includeSpent")]
    pub include_spent: bool,
    #[serde(rename = "outputType")]
    pub output_type: Option<OutputTypeDto>,
}

impl From<AddressOutputsOptionsDto> for AddressOutputsOptions {
    fn from(value: AddressOutputsOptionsDto) -> Self {
        Self {
            include_spent: value.include_spent,
            output_type: value.output_type.map(|o| o.into()),
        }
    }
}

pub struct ClientWrapper(String);

impl Drop for ClientWrapper {
    fn drop(&mut self) {
        crate::remove_client(&self.0);
    }
}

declare_types! {
    pub class JsClient for ClientWrapper {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            Ok(ClientWrapper(client_id))
        }

        ///////////////////////////////////////////////////////////////////////
        // High level API
        ///////////////////////////////////////////////////////////////////////

        method message(mut cx) {
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsMessageSender::new(&mut cx, vec![client_id])?.upcast())
        }

        method getUnspentAddress(mut cx) {
            let seed = cx.argument::<JsString>(0)?;
            // validate the seed
            Seed::from_bytes(&hex::decode(seed.value()).expect("invalid seed hex"));
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsUnspentAddressGetter::new(&mut cx, vec![client_id, seed])?.upcast())
        }

        method getAddresses(mut cx) {
            let seed = cx.argument::<JsString>(0)?;
            // validate the seed
            Seed::from_bytes(&hex::decode(seed.value()).expect("invalid seed hex"));
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsAddressGetter::new(&mut cx, vec![client_id, seed])?.upcast())
        }

        method findMessages(mut cx) {
            let js_indexation_keys: Vec<Handle<JsValue>> = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;
            let js_message_ids: Vec<Handle<JsValue>> = cx.argument::<JsArray>(1)?.to_vec(&mut cx)?;
            let mut indexation_keys = vec![];
            let mut message_ids = vec![];
            for js_message_id in js_message_ids {
                let message_id: Handle<JsString> = js_message_id.downcast_or_throw(&mut cx)?;
                message_ids.push(MessageId::from_str(message_id.value().as_str()).unwrap_or_else(|_| panic!("invalid message id: {}", message_id.value())));
            }
            for js_indexation_key in js_indexation_keys {
                let indexation_key: Handle<JsString> = js_indexation_key.downcast_or_throw(&mut cx)?;
                indexation_keys.push(indexation_key.value());
            }

            let cb = cx.argument::<JsFunction>(2)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::FindMessages { indexation_keys, message_ids },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getBalance(mut cx) {
            let seed = cx.argument::<JsString>(0)?;
            // validate the seed
            Seed::from_bytes(&hex::decode(seed.value()).expect("invalid seed hex"));
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsBalanceGetter::new(&mut cx, vec![client_id, seed])?.upcast())
        }

        method getAddressBalances(mut cx) {
            let js_addresses: Vec<Handle<JsValue>> = cx.argument::<JsArray>(0)?.to_vec(&mut cx)?;
            let mut addresses = vec![];
            for js_address in js_addresses {
                let address: Handle<JsString> = js_address.downcast_or_throw(&mut cx)?;
                addresses.push(address.value());
            }

            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetAddressBalances(addresses),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method retry(mut cx) {
            let message_id = cx.argument::<JsString>(0)?.value();
            let message_id = MessageId::from_str(message_id.as_str()).expect("invalid message id");
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::Retry(message_id),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method retryUntilIncluded(mut cx) {
            let message_id = cx.argument::<JsString>(0)?.value();
            let message_id = MessageId::from_str(message_id.as_str()).expect("invalid message id");
            let interval: Option<u64> = match cx.argument_opt(1) {
                Some(arg) => {
                    Some(arg.downcast::<JsNumber>().or_throw(&mut cx)?.value() as u64)
                },
                None => None,
            };
            let max_attempts: Option<u64> = match cx.argument_opt(2) {
                Some(arg) => {
                    Some(arg.downcast::<JsNumber>().or_throw(&mut cx)?.value() as u64)
                },
                None => None,
            };
            let cb = cx.argument::<JsFunction>(cx.len()-1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::RetryUntilIncluded(message_id, interval, max_attempts),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method consolidateFunds(mut cx) {
            let seed_hex = cx.argument::<JsString>(0)?.value();
            let seed = Seed::from_bytes(&hex::decode(seed_hex).expect("invalid seed hex"));
            let account_index = cx.argument::<JsNumber>(1)?.value() as usize;
            let start_index = cx.argument::<JsNumber>(2)?.value() as usize;
            let end_index = cx.argument::<JsNumber>(3)?.value() as usize;
            let cb = cx.argument::<JsFunction>(cx.len()-1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::ConsolidateFunds(seed, account_index, start_index, end_index),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method networkInfo(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetNetworkInfo,
                };
                client_task.schedule(cb);
            };
            Ok(cx.undefined().upcast())
        }

        ///////////////////////////////////////////////////////////////////////
        // Node API
        ///////////////////////////////////////////////////////////////////////

        method subscriber(mut cx) {
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);
            Ok(crate::JsTopicSubscriber::new(&mut cx, vec![client_id])?.upcast())
        }

        method getInfo(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetInfo,
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getPeers(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetPeers,
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getTips(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetTips,
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method postMessage(mut cx) {
            let message = cx.argument::<JsString>(0)?.value();
            let message: MessageDto = serde_json::from_str(&message).expect("invalid message argument");
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::PostMessage(message),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getMessage(mut cx) {
            let id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.clone()
            };
            let id = cx.string(id);

            Ok(JsMessageGetter::new(&mut cx, vec![id])?.upcast())
        }

        method getOutput(mut cx) {
            let output_id = cx.argument::<JsString>(0)?.value();
            let output_id = UtxoInput::from_str(output_id.as_str()).expect("invalid output id");
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetOutput(output_id),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method findOutputs(mut cx) {
            let js_output_ids = cx.argument::<JsArray>(0)?;
            let js_output_ids: Vec<Handle<JsValue>> = js_output_ids.to_vec(&mut cx)?;
            let mut outputs = vec![];
            for js_output_id in js_output_ids {
                let output_id: Handle<JsString> = js_output_id.downcast_or_throw(&mut cx)?;
                let output_id = UtxoInput::from_str(output_id.value().as_str()).expect("invalid output id");
                outputs.push(output_id);
            }

            let js_addresses = cx.argument::<JsArray>(1)?;
            let js_addresses: Vec<Handle<JsValue>> = js_addresses.to_vec(&mut cx)?;
            let mut addresses = vec![];
            for js_address in js_addresses {
                let address: Handle<JsString> = js_address.downcast_or_throw(&mut cx)?;
                addresses.push(address.value());
            }

            let cb = cx.argument::<JsFunction>(2)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::FindOutputs {
                        outputs,
                        addresses
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getAddressOutputs(mut cx) {
            let address = cx.argument::<JsString>(0)?.value();
            let options: AddressOutputsOptionsDto = match cx.argument_opt(1) {
                Some(arg) => {
                    let json = arg.downcast::<JsString>().or_throw(&mut cx)?.value();
                    serde_json::from_str(&json).expect("invalid options")
                },
                None => Default::default(),
            };

            let cb = cx.argument::<JsFunction>(cx.len()-1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetAddressOutputs(address, options.into()),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getAddressBalance(mut cx) {
            let address = cx.argument::<JsString>(0)?.value();

            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetAddressBalance(address),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getMilestone(mut cx) {
            let milestone_index = cx.argument::<JsNumber>(0)?.value() as u32;

            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetMilestone(milestone_index),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getMilestoneUtxoChanges(mut cx) {
            let milestone_index = cx.argument::<JsNumber>(0)?.value() as u32;

            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetMilestoneUtxoChanges(milestone_index),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getReceipts(mut cx) {

            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetReceipts(),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getReceiptsMigratedAt(mut cx) {
            let milestone_index = cx.argument::<JsNumber>(0)?.value() as u32;

            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetReceiptsMigratedAt(milestone_index),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getTreasury(mut cx) {

            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetTreasury(),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getIncludedMessage(mut cx) {
            let transaction_id = cx.argument::<JsString>(0)?.value();
            let transaction_id = TransactionId::from_str(transaction_id.as_str()).expect("invalid transaction id");

            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetIncludedMessage(transaction_id),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method reattach(mut cx) {
            let message_id = cx.argument::<JsString>(0)?.value();
            let message_id = MessageId::from_str(message_id.as_str()).expect("invalid message id");
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::Reattach(message_id),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method promote(mut cx) {
            let message_id = cx.argument::<JsString>(0)?.value();
            let message_id = MessageId::from_str(message_id.as_str()).expect("invalid message id");
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::Promote(message_id),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method generateMnemonic(mut cx) {
            let mnemonic = Client::generate_mnemonic().unwrap();
            Ok(cx.string(mnemonic).upcast())
        }

        method mnemonicToHexSeed(mut cx) {
            let mnemonic = cx.argument::<JsString>(0)?.value();
            let hex = Client::mnemonic_to_hex_seed(&mnemonic).unwrap();
            Ok(cx.string(hex).upcast())
        }

        method bech32ToHex(mut cx) {
            let bech32 = cx.argument::<JsString>(0)?.value();
            let hex = Client::bech32_to_hex(bech32.as_str()).unwrap();
            Ok(cx.string(hex).upcast())
        }

        method hexToBech32(mut cx) {
            let hex = cx.argument::<JsString>(0)?.value();
            let bech32_hrp: Option<String> = if cx.len() > 2 {
                    match cx.argument_opt(1){
                        Some(arg) => Some(arg.downcast::<JsString>().or_throw(&mut cx)?.value()),
                        None => Default::default(),
                    }
                } else {
                    Default::default()
            };

            let cb = cx.argument::<JsFunction>(cx.len()-1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::HexToBech32(hex, bech32_hrp),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method isAddressValid(mut cx) {
            let address = cx.argument::<JsString>(0)?.value();
            let is_valid = Client::is_address_valid(address.as_str());
            Ok(cx.boolean(is_valid).upcast())
        }
    }
}
