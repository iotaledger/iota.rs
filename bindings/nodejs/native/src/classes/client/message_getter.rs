// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::bee_message::prelude::MessageId;
use neon::prelude::*;

use std::str::FromStr;

use super::{Api, ClientTask};

pub struct MessageGetter(String);

declare_types! {
    pub class JsMessageGetter for MessageGetter {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            Ok(MessageGetter(client_id))
        }

        method index(mut cx) {
            let mut index: Vec<u8> = vec![];
            let index_js_array = cx.argument::<JsArray>(0)?;
            let js_index: Vec<Handle<JsValue>> = index_js_array.to_vec(&mut cx)?;
            for value in js_index {
                let value: Handle<JsNumber> = value.downcast_or_throw(&mut cx)?;
                index.push(value.value() as u8);
            }

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
