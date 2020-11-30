use bech32::FromBase32;
use iota::{
    message::prelude::{Address, Ed25519Address, Message, MessageId, UTXOInput},
    BIP32Path, Seed,
};
use neon::prelude::*;

use std::{
    convert::TryInto,
    num::NonZeroU64,
    ops::Range,
    str::FromStr,
    sync::{Arc, Mutex},
};

mod builder;
pub use builder::*;

/// Parses a bech32 address string.
fn parse_address(address: String) -> crate::Result<Address> {
    let address_ed25519 = Vec::from_base32(&bech32::decode(&address)?.1)?;
    let address = Address::Ed25519(Ed25519Address::new(
        address_ed25519[1..]
            .try_into()
            .map_err(|_| anyhow::anyhow!("invalid address length"))?,
    ));
    Ok(address)
}

enum Api {
    // High level APIs
    SendTransfer {
        seed: Seed,
        path: Option<BIP32Path>,
        index: Option<usize>,
        outputs: Vec<(Address, NonZeroU64)>,
    },
    GetUnspentAddress {
        seed: Seed,
        path: Option<BIP32Path>,
        index: Option<usize>,
    },
    // Node APIs
    GetInfo,
    GetTips,
    PostMessage(Message),
    GetMessagesByIndexation(String),
    GetMessage(MessageId),
    GetMessageMetadata(MessageId),
    GetRawMessage(MessageId),
    GetMessageChildren(MessageId),
    GetOutput(UTXOInput),
    FindOutputs {
        outputs: Vec<UTXOInput>,
        addresses: Vec<Address>,
    },
    GetAddressBalance(Address),
    GetAddressOutputs(Address),
    GetMilestone(u64),
    Retry(MessageId),
    Reattach(MessageId),
    Promote(MessageId),
}

struct ClientTask {
    client_id: String,
    api: Api,
}

impl Task for ClientTask {
    type Output = String;
    type Error = crate::Error;
    type JsEvent = JsString;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        crate::block_on(crate::convert_async_panics(|| async move {
            let client = crate::get_client(self.client_id.clone());
            let client = client.read().unwrap();
            let res = match &self.api {
                Api::SendTransfer {
                    seed,
                    path,
                    index,
                    outputs,
                } => {
                    let mut sender = client.send(seed);
                    if let Some(path) = path {
                        sender = sender.path(path);
                    }
                    if let Some(index) = index {
                        sender = sender.index(*index);
                    }
                    for output in outputs {
                        sender = sender.output(output.0.clone(), output.1);
                    }
                    let message_id = sender.post().await?;
                    serde_json::to_string(&message_id).unwrap()
                }
                Api::GetUnspentAddress { seed, path, index } => {
                    let mut getter = client.get_unspent_address(seed);
                    if let Some(path) = path {
                        getter = getter.path(path);
                    }
                    if let Some(index) = index {
                        getter = getter.index(*index);
                    }
                    let (address, index) = getter.get().await?;
                    serde_json::to_string(&(address, index)).unwrap()
                }
                Api::GetInfo => serde_json::to_string(&client.get_info().await?).unwrap(),
                Api::GetTips => {
                    let tips = client.get_tips().await?;
                    let tips = vec![tips.0, tips.1];
                    serde_json::to_string(&tips).unwrap()
                }
                Api::PostMessage(message) => {
                    let message_id = client.post_message(message).await?;
                    serde_json::to_string(&message_id).unwrap()
                }
                Api::GetMessagesByIndexation(index) => {
                    let messages = client.get_message().index(index.as_str()).await?;
                    serde_json::to_string(&messages).unwrap()
                }
                Api::GetMessage(id) => {
                    let message = client.get_message().data(&id).await?;
                    serde_json::to_string(&message).unwrap()
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
                    serde_json::to_string(&output).unwrap()
                }
                Api::FindOutputs { outputs, addresses } => {
                    let outputs = client.find_outputs(outputs, addresses).await?;
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
                Api::Retry(message_id) => {
                    let message = client.retry(message_id).await?;
                    serde_json::to_string(&message).unwrap()
                }
                Api::Reattach(message_id) => {
                    let message = client.reattach(message_id).await?;
                    serde_json::to_string(&message).unwrap()
                }
                Api::Promote(message_id) => {
                    let message = client.promote(message_id).await?;
                    serde_json::to_string(&message).unwrap()
                }
            };
            Ok(res)
        }))
    }

    fn complete(
        self,
        mut cx: TaskContext,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        match result {
            Ok(s) => Ok(cx.string(s)),
            Err(e) => cx.throw_error(format!("ClientTask error: {:?}", e)),
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
            Seed::from_ed25519_bytes(seed.value().as_bytes()).expect("invalid seed");
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
            Seed::from_ed25519_bytes(seed.value().as_bytes()).expect("invalid seed");
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
            Seed::from_ed25519_bytes(seed.value().as_bytes()).expect("invalid seed");
            let client_id = {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                id.to_string()
            };
            let client_id = cx.string(client_id);

            Ok(JsAddressFinder::new(&mut cx, vec![client_id, seed])?.upcast())
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
            let message: Message = serde_json::from_str(&message).expect("invalid message argument");
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

            Ok(JsMessageFinder::new(&mut cx, vec![id])?.upcast())
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

pub struct ValueTransactionSender {
    client_id: String,
    seed: String,
    path: Arc<Mutex<Option<BIP32Path>>>,
    index: Arc<Mutex<Option<usize>>>,
    outputs: Arc<Mutex<Vec<(Address, NonZeroU64)>>>,
}

declare_types! {
    pub class JsValueTransactionSender for ValueTransactionSender {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(ValueTransactionSender {
                client_id,
                seed,
                path: Arc::new(Mutex::new(None)),
                index: Arc::new(Mutex::new(None)),
                outputs: Arc::new(Mutex::new(vec![]))
            })
        }

        method path(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            let path = BIP32Path::from_str(path.as_str()).expect("invalid bip32 path");
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).path;
                let mut send_path = ref_.lock().unwrap();
                send_path.replace(path);
            }

            Ok(cx.this().upcast())
        }

        method index(mut cx) {
            let index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).index;
                let mut send_index = ref_.lock().unwrap();
                send_index.replace(index);
            }

            Ok(cx.this().upcast())
        }

        method output(mut cx) {
            let address = cx.argument::<JsString>(0)?.value();
            let address = parse_address(address).expect("invalid address");
            let value = cx.argument::<JsNumber>(1)?.value() as u64;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).outputs;
                let mut outputs = ref_.lock().unwrap();
                outputs.push((address, NonZeroU64::new(value).expect("value can't be zero")));
            }

            Ok(cx.this().upcast())
        }

        method send(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard));
                let client_task = ClientTask {
                    client_id: ref_.client_id.clone(),
                    api: Api::SendTransfer {
                        seed: Seed::from_ed25519_bytes(ref_.seed.as_bytes()).expect("invalid seed"),
                        path: (*ref_.path.lock().unwrap()).clone(),
                        index: *ref_.index.lock().unwrap(),
                        outputs: (*ref_.outputs.lock().unwrap()).clone(),
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}

pub struct UnspentAddressGetter {
    client_id: String,
    seed: String,
    path: Arc<Mutex<Option<BIP32Path>>>,
    index: Arc<Mutex<Option<usize>>>,
}

declare_types! {
    pub class JsUnspentAddressGetter for UnspentAddressGetter {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(UnspentAddressGetter {
                client_id,
                seed,
                path: Arc::new(Mutex::new(None)),
                index: Arc::new(Mutex::new(None)),
            })
        }

        method path(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            let path = BIP32Path::from_str(path.as_str()).expect("invalid bip32 path");
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).path;
                let mut get_path = ref_.lock().unwrap();
                get_path.replace(path);
            }

            Ok(cx.this().upcast())
        }

        method index(mut cx) {
            let index = cx.argument::<JsNumber>(0)?.value() as usize;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).index;
                let mut get_index = ref_.lock().unwrap();
                get_index.replace(index);
            }

            Ok(cx.this().upcast())
        }

        method get(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard));
                let client_task = ClientTask {
                    client_id: ref_.client_id.clone(),
                    api: Api::GetUnspentAddress {
                        seed: Seed::from_ed25519_bytes(ref_.seed.as_bytes()).expect("invalid seed"),
                        path: (*ref_.path.lock().unwrap()).clone(),
                        index: *ref_.index.lock().unwrap(),
                    },
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}

pub struct AddressFinder {
    client_id: String,
    seed: String,
    path: Arc<Mutex<Option<BIP32Path>>>,
    range: Arc<Mutex<Option<Range<usize>>>>,
}

declare_types! {
    pub class JsAddressFinder for AddressFinder {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let seed = cx.argument::<JsString>(1)?.value();
            Ok(AddressFinder {
                client_id,
                seed,
                path: Arc::new(Mutex::new(None)),
                range: Arc::new(Mutex::new(None)),
            })
        }

        method path(mut cx) {
            let path = cx.argument::<JsString>(0)?.value();
            let path = BIP32Path::from_str(path.as_str()).expect("invalid bip32 path");
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).path;
                let mut find_path = ref_.lock().unwrap();
                find_path.replace(path);
            }

            Ok(cx.this().upcast())
        }

        method range(mut cx) {
            let start = cx.argument::<JsNumber>(0)?.value() as usize;
            let end = cx.argument::<JsNumber>(1)?.value() as usize;
            let range = Range { start, end };
            {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard)).range;
                let mut find_range = ref_.lock().unwrap();
                find_range.replace(range);
            }

            Ok(cx.this().upcast())
        }

        method get(mut cx) {
            let addresses_json = {
                let this = cx.this();
                let guard = cx.lock();
                let ref_ = &(*this.borrow(&guard));

                let seed = Seed::from_ed25519_bytes(ref_.seed.as_bytes()).expect("invalid seed");

                let client = crate::get_client(ref_.client_id.clone());
                let client = client.read().unwrap();
                let mut getter = client.find_addresses(&seed);

                let path = &*ref_.path.lock().unwrap();
                if let Some(path) = path {
                    getter = getter.path(&path);
                }
                if let Some(range) = &*ref_.range.lock().unwrap() {
                    getter = getter.range(range.clone());
                }
                getter.get().map(|addresses| serde_json::to_string(&addresses).unwrap())
            };

            match addresses_json {
                Ok(addresses) => Ok(cx.string(addresses).upcast()),
                Err(e) => cx.throw_error(e.to_string()),
            }
        }
    }
}

pub struct MessageFinder(String);

declare_types! {
    pub class JsMessageFinder for MessageFinder {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            Ok(MessageFinder(client_id))
        }

        method index(mut cx) {
            let index = cx.argument::<JsString>(0)?.value();
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetMessagesByIndexation(index),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method data(mut cx) {
            let message_id = cx.argument::<JsString>(0)?.value();
            let message_id = MessageId::from_str(message_id.as_str()).expect("invalid message id");
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetMessage(message_id),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method raw(mut cx) {
            let message_id = cx.argument::<JsString>(0)?.value();
            let message_id = MessageId::from_str(message_id.as_str()).expect("invalid message id");
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetRawMessage(message_id),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method children(mut cx) {
            let message_id = cx.argument::<JsString>(0)?.value();
            let message_id = MessageId::from_str(message_id.as_str()).expect("invalid message id");
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetMessageChildren(message_id),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }

        method metadata(mut cx) {
            let message_id = cx.argument::<JsString>(0)?.value();
            let message_id = MessageId::from_str(message_id.as_str()).expect("invalid message id");
            let cb = cx.argument::<JsFunction>(1)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let id = &this.borrow(&guard).0;
                let client_task = ClientTask {
                    client_id: id.clone(),
                    api: Api::GetMessageMetadata(message_id),
                };
                client_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}
