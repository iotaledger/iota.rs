// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bech32::FromBase32;
use iota::{
    message::prelude::{Address, Ed25519Address, MessageId, UTXOInput},
    Seed,
};
use neon::prelude::*;

use std::{convert::TryInto, str::FromStr};

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

mod address_finder;
pub use address_finder::JsAddressFinder;

mod balance_getter;
pub use balance_getter::JsBalanceGetter;

fn parse_bech32_address(address: String) -> crate::Result<Address> {
    let address_ed25519 = Vec::from_base32(&bech32::decode(&address)?.1)?;
    let address = Address::Ed25519(Ed25519Address::new(
        address_ed25519[1..]
            .try_into()
            .map_err(|_| anyhow::anyhow!("invalid address length"))?,
    ));
    Ok(address)
}

/// Parses a bech32 address string.
fn parse_address(address: String) -> crate::Result<Address> {
    match parse_bech32_address(address.clone()) {
        Ok(address) => Ok(address),
        Err(_) => Ok(Address::Ed25519(Ed25519Address::new(
            hex::decode(address)?.try_into().expect("invalid address length"),
        ))),
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
            Seed::from_bytes(&hex::decode(seed.value()).expect("invalid seed hex")).expect("invalid seed");
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsUnspentAddressGetter::new(&mut cx, vec![client_id, seed])?.upcast())
        }

        method findAddresses(mut cx) {
            let seed = cx.argument::<JsString>(0)?;
            // validate the seed
            Seed::from_bytes(&hex::decode(seed.value()).expect("invalid seed hex")).expect("invalid seed");
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsAddressFinder::new(&mut cx, vec![client_id, seed])?.upcast())
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
            Seed::from_bytes(&hex::decode(seed.value()).expect("invalid seed hex")).expect("invalid seed");
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
                addresses.push(address.value().into());
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
            let output_id = UTXOInput::from_str(output_id.as_str()).expect("invalid output id");
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
                let output_id = UTXOInput::from_str(output_id.value().as_str()).expect("invalid output id");
                outputs.push(output_id);
            }

            let js_addresses = cx.argument::<JsArray>(1)?;
            let js_addresses: Vec<Handle<JsValue>> = js_addresses.to_vec(&mut cx)?;
            let mut addresses = vec![];
            for js_address in js_addresses {
                let address: Handle<JsString> = js_address.downcast_or_throw(&mut cx)?;
                addresses.push(address.value().into());
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

            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetAddressOutputs(address.into()),
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
                    api: Api::GetAddressBalance(address.into()),
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

        method getMilestoneUTXOChanges(mut cx) {
            let milestone_index = cx.argument::<JsNumber>(0)?.value() as u32;

            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetMilestoneUTXOChanges(milestone_index),
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
    }
}
