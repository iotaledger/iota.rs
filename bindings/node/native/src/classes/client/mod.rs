use bech32::FromBase32;
use iota::message::prelude::{Address, Ed25519Address, Message, MessageId, UTXOInput};
use neon::prelude::*;

use std::{convert::TryInto, str::FromStr};

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
                Api::GetInfo => serde_json::to_string(&client.get_info().await?).unwrap(),
                Api::GetTips => {
                    let tips = client.get_tips().await?;
                    let tips = vec![tips.0.to_string(), tips.1.to_string()];
                    serde_json::to_string(&tips).unwrap()
                }
                Api::PostMessage(message) => {
                    let message_id = client.post_message(message).await?;
                    serde_json::to_string(&message_id).unwrap()
                }
                Api::GetMessagesByIndexation(index) => {
                    let messages = client.get_message().index(index.as_str()).await?;
                    let messages_str: Vec<String> =
                        messages.iter().map(|m| m.to_string()).collect();
                    serde_json::to_string(&messages_str).unwrap()
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
                    let messages_str: Vec<String> =
                        messages.iter().map(|m| m.to_string()).collect();
                    serde_json::to_string(&messages_str).unwrap()
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
