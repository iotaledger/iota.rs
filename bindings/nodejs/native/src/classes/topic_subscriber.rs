// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::{
    convert::TryInto,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc, Mutex,
    },
};

use iota_client::Topic;
use neon::prelude::*;

struct WaitForMessageTask(Arc<Mutex<Receiver<String>>>);

impl Task for WaitForMessageTask {
    type Output = String;
    type Error = crate::Error;
    type JsEvent = JsString;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        crate::convert_panics(|| {
            let rx = self
                .0
                .lock()
                .map_err(|_| anyhow::anyhow!("Could not obtain lock on receiver"))?;
            let res = rx.recv().map_err(|e| anyhow::anyhow!(e.to_string()))?;
            Ok(res)
        })
    }

    fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        match result {
            Ok(s) => Ok(cx.string(s)),
            Err(e) => cx.throw_error(format!("WaitForMessageTask error: {:?}", e)),
        }
    }
}

enum TopicAction {
    Subscribe,
    Unsubscribe,
}

struct TopicTask {
    client_id: String,
    topics: Vec<Topic>,
    action: TopicAction,
    sender: Sender<String>,
}

impl Task for TopicTask {
    type Output = ();
    type Error = crate::Error;
    type JsEvent = JsUndefined;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        crate::convert_panics(|| {
            let sender = Arc::new(Mutex::new(self.sender.clone()));
            let client = crate::get_client(&self.client_id);
            let mut client = client.write().unwrap();
            match self.action {
                TopicAction::Subscribe => {
                    crate::block_on(
                        client
                            .subscriber()
                            .with_topics(self.topics.clone())
                            .subscribe(move |event| {
                                let s = sender.lock().unwrap();
                                let _ = s.send(serde_json::to_string(&event).unwrap());
                            }),
                    )
                    .expect("failed to subscribe to topics");
                }
                TopicAction::Unsubscribe => {
                    crate::block_on(client.subscriber().with_topics(self.topics.clone()).unsubscribe())
                        .expect("failed to unsbuscribe from topics");
                }
            }
            Ok(())
        })
    }

    fn complete(self, mut cx: TaskContext, result: Result<Self::Output, Self::Error>) -> JsResult<Self::JsEvent> {
        match result {
            Ok(_) => Ok(cx.undefined()),
            Err(e) => cx.throw_error(format!("SubUnsubTask error: {:?}", e)),
        }
    }
}

pub struct TopicSubscriber {
    tx: Sender<String>,
    rx: Arc<Mutex<Receiver<String>>>,
    client_id: String,
    topics: Vec<Topic>,
}

declare_types! {
    pub class JsTopicSubscriber for TopicSubscriber {
        init(mut cx) {
            let client_id = cx.argument::<JsString>(0)?.value();
            let (tx, rx) = channel();

            Ok(TopicSubscriber {
                tx,
                rx: Arc::new(Mutex::new(rx)),
                client_id,
                topics: Vec::new(),
            })
        }

        method topic(mut cx) {
            let js_topic = cx.argument::<JsString>(0)?;
            let topic = js_topic.value().as_str().try_into().unwrap_or_else(|_| panic!("invalid topic: {}", js_topic.value()));

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let topics = &mut this.borrow_mut(&guard).topics;
                topics.push(topic);
            }

            Ok(cx.this().upcast())
        }

        method topics(mut cx) {
            let mut topics: Vec<Topic> = vec![];

            let topic_js_array = cx.argument::<JsArray>(0)?;
            let js_topics: Vec<Handle<JsValue>> = topic_js_array.to_vec(&mut cx)?;
            for js_topic in js_topics {
                let topic: Handle<JsString> = js_topic.downcast_or_throw(&mut cx)?;
                topics.push(topic.value().as_str().try_into().unwrap_or_else(|_| panic!("invalid topic: {}", topic.value())));
            }

            {
                let mut this = cx.this();
                let guard = cx.lock();
                let stored_topics = &mut this.borrow_mut(&guard).topics;
                stored_topics.extend(topics.into_iter());
            }

            Ok(cx.this().upcast())
        }

        method subscribe(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let instance = &this.borrow(&guard);
                let topic_task = TopicTask {
                    client_id: instance.client_id.clone(),
                    topics: instance.topics.clone(),
                    action: TopicAction::Subscribe,
                    sender: instance.tx.clone(),
                };
                topic_task.schedule(cb);
            }

            Ok(cx.this().upcast())
        }

        method unsubscribe(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;
            {
                let this = cx.this();
                let guard = cx.lock();
                let instance = &this.borrow(&guard);
                let topic_task = TopicTask {
                    client_id: instance.client_id.clone(),
                    topics: instance.topics.clone(),
                    action: TopicAction::Unsubscribe,
                    sender: instance.tx.clone(),
                };
                topic_task.schedule(cb);
            }
            Ok(cx.this().upcast())
        }

        method poll(mut cx) {
            let cb = cx.argument::<JsFunction>(0)?;

            {
                let this = cx.this();
                let rx = cx.borrow(&this, |subscriber| subscriber.rx.clone());
                let receive_task = WaitForMessageTask(rx);
                receive_task.schedule(cb);
            }

            Ok(cx.undefined().upcast())
        }
    }
}
