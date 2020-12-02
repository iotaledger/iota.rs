use bech32::FromBase32;
use iota::{
    message::prelude::{
        Address, Ed25519Address, Ed25519Signature, Indexation, Input, Message, MessageId, Output,
        Payload, ReferenceUnlock, SignatureLockedSingleOutput, SignatureUnlock, Transaction,
        TransactionEssence, UTXOInput, UnlockBlock,
    },
    Seed,
};
use neon::prelude::*;
use serde::{Deserialize, Serialize};

use std::{
    convert::{TryFrom, TryInto},
    num::NonZeroU64,
    str::FromStr,
};

mod builder;
pub use builder::*;

mod api;
use api::{Api, ClientTask};

mod message_getter;
pub use message_getter::JsMessageGetter;

mod value_transaction_sender;
pub use value_transaction_sender::JsValueTransactionSender;

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
            hex::decode(address)?
                .try_into()
                .expect("invalid address length"),
        ))),
    }
}

#[derive(Serialize, Deserialize)]
struct OutputDto {
    address: String,
    amount: u64,
}

#[derive(Serialize, Deserialize)]
struct MessageTransactionEssenceDto {
    inputs: Box<[String]>,
    outputs: Box<[OutputDto]>,
    payload: Option<Box<MessagePayloadDto>>,
}

impl TryFrom<MessageTransactionEssenceDto> for TransactionEssence {
    type Error = crate::Error;
    fn try_from(value: MessageTransactionEssenceDto) -> crate::Result<Self> {
        let mut builder = TransactionEssence::builder();

        let inputs: Vec<Input> = value
            .inputs
            .into_vec()
            .into_iter()
            .map(|input| {
                UTXOInput::from_str(&input)
                    .unwrap_or_else(|_| panic!("invalid input: {}", input))
                    .into()
            })
            .collect();
        for input in inputs {
            builder = builder.add_input(input);
        }

        let outputs: Vec<Output> = value
            .outputs
            .into_vec()
            .into_iter()
            .map(|output| {
                SignatureLockedSingleOutput::new(
                    parse_address(output.address.clone())
                        .unwrap_or_else(|_| panic!("invalid output address: {}", output.address)),
                    NonZeroU64::new(output.amount).expect("output amount can't be zero"),
                )
                .into()
            })
            .collect();
        for output in outputs {
            builder = builder.add_output(output);
        }

        builder = match value.payload {
            Some(indexation) => builder.with_payload(
                (*indexation)
                    .try_into()
                    .expect("Invalid indexation in TransactionEssenceJson"),
            ),
            _ => builder,
        };

        Ok(builder.finish()?)
    }
}

#[derive(Serialize, Deserialize)]
struct MessageSignatureUnlockDto {
    #[serde(rename = "publicKey")]
    public_key: String,
    signature: String,
}

impl TryFrom<MessageSignatureUnlockDto> for SignatureUnlock {
    type Error = crate::Error;

    fn try_from(value: MessageSignatureUnlockDto) -> crate::Result<Self> {
        let mut public_key = [0u8; 32];
        hex::decode_to_slice(value.public_key, &mut public_key)?;
        let signature = hex::decode(value.signature)?.into_boxed_slice();
        Ok(Ed25519Signature::new(public_key, signature).into())
    }
}

#[derive(Serialize, Deserialize)]
struct MessageUnlockBlockJsonDto {
    signature: Option<MessageSignatureUnlockDto>,
    reference: Option<u16>,
}

impl TryFrom<MessageUnlockBlockJsonDto> for UnlockBlock {
    type Error = crate::Error;

    fn try_from(value: MessageUnlockBlockJsonDto) -> crate::Result<Self> {
        let type_ = if value.signature.is_some() { 0 } else { 1 };
        match type_ {
            0 => {
                let sig: SignatureUnlock = value
                    .signature
                    .expect("Must contain signature.")
                    .try_into()?;
                Ok(sig.into())
            }
            1 => {
                let reference: ReferenceUnlock = value
                    .reference
                    .expect("Must contain reference.")
                    .try_into()?;
                Ok(reference.into())
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Serialize, Deserialize)]
struct MessageTransactionPayloadDto {
    essence: MessageTransactionEssenceDto,
    #[serde(rename = "unlockBlocks")]
    unlock_blocks: Box<[MessageUnlockBlockJsonDto]>,
}

#[derive(Serialize, Deserialize)]
struct MessageIndexationPayloadDto {
    index: String,
    data: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum MessagePayloadDto {
    /// The transaction payload.
    Transaction(MessageTransactionPayloadDto),
    /// The indexation payload.
    Indexation(MessageIndexationPayloadDto),
}

#[derive(Serialize, Deserialize)]
struct MessageDto {
    parent1: String,
    parent2: String,
    payload: MessagePayloadDto,
    nonce: u64,
}

impl TryFrom<MessagePayloadDto> for Payload {
    type Error = crate::Error;
    fn try_from(payload: MessagePayloadDto) -> crate::Result<Self> {
        match payload {
            MessagePayloadDto::Transaction(transaction_payload) => {
                let mut transaction = Transaction::builder();
                transaction = transaction.with_essence(transaction_payload.essence.try_into()?);

                let unlock_blocks = transaction_payload.unlock_blocks.into_vec();
                for unlock_block in unlock_blocks {
                    transaction = transaction.add_unlock_block(unlock_block.try_into()?);
                }

                Ok(Payload::Transaction(Box::new(transaction.finish()?)))
            }
            MessagePayloadDto::Indexation(indexation_payload) => {
                let indexation =
                    Indexation::new(indexation_payload.index, indexation_payload.data.as_bytes())
                        .unwrap();
                Ok(Payload::Indexation(Box::new(indexation)))
            }
        }
    }
}

pub struct ClientWrapper(String);

impl Drop for ClientWrapper {
    fn drop(&mut self) {
        crate::remove_client(self.0.clone());
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

        method send(mut cx) {
            let seed = cx.argument::<JsString>(0)?;
            // validate the seed
            Seed::from_ed25519_bytes(&hex::decode(seed.value()).expect("invalid seed hex")).expect("invalid seed");
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsValueTransactionSender::new(&mut cx, vec![client_id, seed])?.upcast())
        }

        method getUnspentAddress(mut cx) {
            let seed = cx.argument::<JsString>(0)?;
            // validate the seed
            Seed::from_ed25519_bytes(&hex::decode(seed.value()).expect("invalid seed hex")).expect("invalid seed");
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
            Seed::from_ed25519_bytes(&hex::decode(seed.value()).expect("invalid seed hex")).expect("invalid seed");
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
            let mut indexation_keys = vec![];
            for js_indexation_key in js_indexation_keys {
                let indexation_key: Handle<JsString> = js_indexation_key.downcast_or_throw(&mut cx)?;
                indexation_keys.push(indexation_key.value());
            }

            let js_message_ids: Vec<Handle<JsValue>> = cx.argument::<JsArray>(1)?.to_vec(&mut cx)?;
            let mut message_ids = vec![];
            for js_message_id in js_message_ids {
                let message_id: Handle<JsString> = js_message_id.downcast_or_throw(&mut cx)?;
                message_ids.push(MessageId::from_str(message_id.value().as_str()).unwrap_or_else(|_| panic!("invalid message id: {}", message_id.value())));
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
            Seed::from_ed25519_bytes(&hex::decode(seed.value()).expect("invalid seed hex")).expect("invalid seed");
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
                addresses.push(parse_address(address.value()).unwrap_or_else(|_| panic!("invalid address: {}", address.value())));
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
            let message_builder = Message::builder()
                 .with_network_id(0)
                 .with_parent1(MessageId::from_str(&message.parent1).expect("invalid parent1 message id"))
                 .with_parent2(MessageId::from_str(&message.parent2).expect("invalid parent2 message id"))
                 .with_nonce(message.nonce)
                 .with_payload(message.payload.try_into().expect("invalid payload"));

            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::PostMessage(message_builder.finish().expect("error building message")),
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
                let address = parse_address(address.value()).expect("invalid address");
                addresses.push(address);
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
            let address = parse_address(address).expect("invalid output id");

            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetAddressOutputs(address),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method getAddressBalance(mut cx) {
            let address = cx.argument::<JsString>(0)?.value();
            let address = parse_address(address).expect("invalid output id");

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
            let milestone_index = cx.argument::<JsNumber>(0)?.value() as u64;

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
