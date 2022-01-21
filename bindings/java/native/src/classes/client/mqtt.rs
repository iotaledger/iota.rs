// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use iota_client::{
    node::{MqttManager as RustMqttManager, MqttTopicManager as RustMqttTopicManager, Topic},
    TopicEvent,
};

use crate::{full_node_api::Client, Result};

use anyhow::anyhow;

pub trait MqttListener {
    fn on_event(&self, event: TopicEvent);
}

pub struct MqttManager<'a> {
    client: &'a mut Client,
}

impl<'a> MqttManager<'a> {
    pub fn new(client: &'a mut Client) -> Self {
        Self { client }
    }

    pub fn with_topic(&mut self, topic: Topic) -> MqttTopicManager {
        MqttTopicManager::new(RustMqttManager::new(self.client.borrow_mut()).with_topic(topic))
    }

    pub fn with_topics(&mut self, topics: Vec<Topic>) -> MqttTopicManager {
        MqttTopicManager::new(RustMqttManager::new(self.client.borrow_mut()).with_topics(topics))
    }

    pub fn unsubscribe(&mut self) -> Result<()> {
        let res = crate::block_on(async { RustMqttManager::new(self.client.borrow_mut()).unsubscribe().await });
        match res {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn disconnect(&mut self) -> Result<()> {
        let res = crate::block_on(async { RustMqttManager::new(self.client.borrow_mut()).disconnect().await });
        match res {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}

pub struct MqttTopicManager<'a>(Rc<RefCell<Option<RustMqttTopicManager<'a>>>>);

impl<'a> MqttTopicManager<'a> {
    pub fn new(manager: RustMqttTopicManager<'a>) -> Self {
        Self(Rc::new(RefCell::new(Option::from(manager))))
    }

    pub fn with_topic(&mut self, topic: Topic) -> Self {
        let new_manager = self.0.borrow_mut().take().unwrap().with_topic(topic);
        MqttTopicManager::new(new_manager)
    }

    pub fn with_topics(&mut self, topics: Vec<Topic>) -> Self {
        let new_manager = self.0.borrow_mut().take().unwrap().with_topics(topics);
        MqttTopicManager::new(new_manager)
    }

    pub fn subscribe(&mut self, cb: Box<dyn MqttListener + Send + Sync + 'static>) -> Result<()> {
        let new_manager = self.0.borrow_mut().take().unwrap();

        let res = crate::block_on(async {
            new_manager
                .subscribe(move |event| {
                    cb.on_event(event.clone());
                })
                .await
        });

        match res {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }

    pub fn unsubscribe(&mut self) -> Result<()> {
        let new_manager = self.0.borrow_mut().take().unwrap();
        let res = crate::block_on(async move { new_manager.unsubscribe().await });
        match res {
            Ok(()) => Ok(()),
            Err(e) => Err(anyhow!(e.to_string())),
        }
    }
}
