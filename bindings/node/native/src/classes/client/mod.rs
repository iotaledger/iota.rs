use iota::{message::prelude::MessageId, ClientBuilder};
use neon::prelude::*;

use std::str::FromStr;

enum Api {
    GetInfo,
    GetTips,
    GetMessagesByIndexation(String),
    GetMessage(MessageId),
    GetMessageMetadata(MessageId),
    GetRawMessage(MessageId),
    GetMessageChildren(MessageId),
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
            let mut client_builder = ClientBuilder::new();
            let node_arg = cx.argument::<JsValue>(0)?;

            if let Ok(node) = node_arg.downcast::<JsString>() {
                client_builder = client_builder.node(&node.value()).expect(&format!("invalid node url: `{}`", node.value()));
            } else if let Ok(node_js_array) = node_arg.downcast::<JsArray>() {
                 let nodes: Vec<Handle<JsValue>> = node_js_array.to_vec(&mut cx)?;
                 for node in nodes {
                      let node: Handle<JsString> = node.downcast_or_throw(&mut cx)?;
                      client_builder = client_builder.node(&node.value()).expect(&format!("invalid node url: `{}`", node.value()));
                 }
            } else {
                return cx.throw_error("invalid node type, expected string or array of strings");
            }

            let client = client_builder.build().expect("failed to build client instance");
            let id = crate::store_client(client);
            Ok(ClientWrapper(id))
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
