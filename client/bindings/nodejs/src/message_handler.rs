// Copyright 2021-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use iota_client::{
    message_interface::{create_message_handler, ClientMessageHandler, Message, Response},
    mqtt::Topic,
};
use neon::prelude::*;

type JsCallback = Root<JsFunction<JsObject>>;

pub struct MessageHandler {
    channel: Channel,
    client_message_handler: ClientMessageHandler,
}

impl Finalize for MessageHandler {}

impl MessageHandler {
    fn new(channel: Channel, options: String) -> Arc<Self> {
        let client_message_handler = create_message_handler(Some(options)).expect("error initializing account manager");

        Arc::new(Self {
            channel,
            client_message_handler,
        })
    }

    async fn send_message(&self, serialized_message: String) -> (String, bool) {
        match serde_json::from_str::<Message>(&serialized_message) {
            Ok(message) => {
                let res = self.client_message_handler.send_message(message).await;
                let mut is_err = matches!(res, Response::Error(_) | Response::Panic(_));

                let msg = match serde_json::to_string(&res) {
                    Ok(msg) => msg,
                    Err(e) => {
                        is_err = true;
                        serde_json::to_string(&Response::Error(e.into()))
                            .expect("the response is generated manually, so unwrap is safe.")
                    }
                };

                (msg, is_err)
            }
            Err(e) => {
                log::debug!("{:?}", e);
                (format!("Couldn't parse to message with error - {e:?}"), true)
            }
        }
    }
}

pub fn message_handler_new(mut cx: FunctionContext) -> JsResult<JsBox<Arc<MessageHandler>>> {
    let options = cx.argument::<JsString>(0)?;
    let options = options.value(&mut cx);
    let channel = cx.channel();
    let message_handler = MessageHandler::new(channel, options);

    Ok(cx.boxed(message_handler))
}

pub fn send_message(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let message = cx.argument::<JsString>(0)?;
    let message = message.value(&mut cx);
    let message_handler = Arc::clone(&&cx.argument::<JsBox<Arc<MessageHandler>>>(1)?);
    let callback = cx.argument::<JsFunction>(2)?.root(&mut cx);

    crate::RUNTIME.spawn(async move {
        let (response, is_error) = message_handler.send_message(message).await;
        message_handler.channel.send(move |mut cx| {
            let cb = callback.into_inner(&mut cx);
            let this = cx.undefined();

            let args = vec![
                if is_error {
                    cx.string(response.clone()).upcast::<JsValue>()
                } else {
                    cx.undefined().upcast::<JsValue>()
                },
                cx.string(response).upcast::<JsValue>(),
            ];

            cb.call(&mut cx, this, args)?;

            Ok(())
        });
    });

    Ok(cx.undefined())
}

// MQTT
pub fn listen(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let js_arr_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_arr_handle.to_vec(&mut cx)?;
    let mut topics = vec![];
    for topic_string in vec {
        let topic = topic_string.downcast::<JsString, FunctionContext>(&mut cx).unwrap();
        topics.push(Topic::try_from(topic.value(&mut cx).as_str().to_string()).expect("invalid MQTT topic"));
    }

    let callback = Arc::new(cx.argument::<JsFunction>(1)?.root(&mut cx));
    let message_handler = Arc::clone(&&cx.argument::<JsBox<Arc<MessageHandler>>>(2)?);
    let (deferred, promise) = cx.promise();

    crate::RUNTIME.spawn(async move {
        let channel0 = message_handler.channel.clone();
        let channel1 = message_handler.channel.clone();
        message_handler
            .client_message_handler
            .listen(topics, move |event_data| {
                call_event_callback(&channel0, event_data, callback.clone())
            })
            .await;

        deferred.settle_with(&channel1, move |mut cx| Ok(cx.undefined()));
    });

    Ok(promise)
}

fn call_event_callback(channel: &neon::event::Channel, event_data: String, callback: Arc<JsCallback>) {
    channel.send(move |mut cx| {
        let cb = (*callback).to_inner(&mut cx);
        let this = cx.undefined();
        let args = vec![
            cx.undefined().upcast::<JsValue>(),
            cx.string(event_data).upcast::<JsValue>(),
        ];

        cb.call(&mut cx, this, args)?;

        Ok(())
    });
}
