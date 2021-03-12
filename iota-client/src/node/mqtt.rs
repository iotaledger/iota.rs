// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{Client, TopicEvent, TopicHandlerMap},
    Result,
};
use bee_common::packable::Packable;
use bee_message::Message;
use regex::Regex;
use rumqttc::{AsyncClient as MqttClient, Event, EventLoop, Incoming, MqttOptions, QoS, SubscribeFilter};
use tokio::sync::RwLock;

use std::{convert::TryFrom, sync::Arc};

macro_rules! lazy_static {
    ($init:expr => $type:ty) => {{
        static mut VALUE: Option<$type> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        INIT.call_once(|| unsafe { VALUE = Some($init) });
        unsafe { VALUE.as_ref() }.expect("failed to get lazy static value")
    }};
}

/// A topic.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Topic(String);

impl TryFrom<&str> for Topic {
    type Error = crate::Error;

    fn try_from(value: &str) -> std::result::Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl Topic {
    /// Creates a new topic and checks if it's valid.
    pub fn new<S: Into<String>>(name: S) -> Result<Self> {
        let valid_topics = lazy_static!(
          ["milestones/latest", "milestones/confirmed", "messages", "messages/referenced"].to_vec() => Vec<&str>
        );
        let regexes = lazy_static!(
          [
            Regex::new(r"messages/([A-Fa-f0-9]{64})/metadata").unwrap(),
            Regex::new(r"outputs/([A-Fa-f0-9]{64})(\d{4})").unwrap(),
            // bech32 address
            Regex::new("addresses/(iota|atoi|iot|toi)1[A-Za-z0-9]+/outputs").unwrap(),
            // ED25519 address hex
            Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})/outputs").unwrap(),
            Regex::new(r"messages/indexation/([a-f0-9]{2,128})").unwrap()
          ].to_vec() => Vec<Regex>
        );
        let name = name.into();
        if valid_topics.iter().any(|valid| valid == &name) || regexes.iter().any(|re| re.is_match(&name)) {
            let topic = Self(name);
            Ok(topic)
        } else {
            Err(crate::Error::InvalidMqttTopic(name))
        }
    }
}

async fn get_mqtt_client(client: &mut Client) -> Result<&mut MqttClient> {
    match client.mqtt_client {
        Some(ref mut c) => Ok(c),
        None => {
            for node in client.sync.read().await.iter() {
                let host = node.host_str().unwrap();
                let mut mqttoptions = MqttOptions::new(host, host, 1883);
                mqttoptions.set_connection_timeout(client.broker_options.timeout.as_secs());
                let (_, mut connection) = MqttClient::new(mqttoptions.clone(), 10);
                // poll the event loop until we find a ConnAck event,
                // which means that the mqtt client is ready to be used on this host
                // if the event loop returns an error, we check the next node
                let mut got_ack = false;
                for event in connection.poll().await {
                    if let Event::Incoming(incoming) = event {
                        if let Incoming::ConnAck(_) = incoming {
                            got_ack = true;
                            break;
                        }
                    }
                }

                // if we found a valid mqtt connection, loop it on a separate thread
                if got_ack {
                    let (mqtt_client, connection) = MqttClient::new(mqttoptions, 10);
                    client.mqtt_client = Some(mqtt_client);
                    poll_mqtt(client.mqtt_topic_handlers.clone(), connection);
                }
            }
            client.mqtt_client.as_mut().ok_or(crate::Error::MqttConnectionNotFound)
        }
    }
}

fn poll_mqtt(mqtt_topic_handlers: Arc<RwLock<TopicHandlerMap>>, mut connection: EventLoop) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async move {
            for event in connection.poll().await {
                let mqtt_topic_handlers = mqtt_topic_handlers.clone();
                if let Event::Incoming(Incoming::Publish(p)) = event {
                    let topic = p.topic.clone();
                    crate::async_runtime::spawn(async move {
                        let mqtt_topic_handlers_guard = mqtt_topic_handlers.read().await;
                        if let Some(handlers) = mqtt_topic_handlers_guard.get(&Topic(topic.clone())) {
                            let event_payload = String::from_utf8_lossy(&*p.payload).to_string();
                            let event = {
                                if topic.as_str() == "messages" || topic.contains("messages/indexation/") {
                                    let mut payload = &*p.payload;
                                    let iota_message = Message::unpack(&mut payload).unwrap();
                                    TopicEvent {
                                        topic,
                                        payload: serde_json::to_string(&iota_message).unwrap(),
                                    }
                                } else {
                                    TopicEvent {
                                        topic,
                                        payload: event_payload,
                                    }
                                }
                            };
                            for handler in handlers {
                                handler(&event)
                            }
                        }
                    });
                }
            }
        });
    });
}

/// MQTT subscriber.
pub struct MqttManager<'a> {
    client: &'a mut Client,
}

impl<'a> MqttManager<'a> {
    /// Initializes a new instance of the mqtt subscriber.
    pub fn new(client: &'a mut Client) -> Self {
        Self { client }
    }

    /// Add a new topic to the list.
    pub fn with_topic(self, topic: Topic) -> MqttTopicManager<'a> {
        MqttTopicManager::new(self.client).with_topic(topic)
    }

    /// Add a collection of topics to the list.
    pub fn with_topics(self, topics: Vec<Topic>) -> MqttTopicManager<'a> {
        MqttTopicManager::new(self.client).with_topics(topics)
    }

    /// Unsubscribes from all subscriptions.
    pub async fn unsubscribe(self) -> crate::Result<()> {
        MqttTopicManager::new(self.client).unsubscribe().await
    }

    /// Disconnects the broker.
    /// This will clear the stored topic handlers and close the MQTT connection.
    pub async fn disconnect(self) -> Result<()> {
        if let Some(client) = &self.client.mqtt_client {
            client.disconnect().await?;
            self.client.mqtt_client = None;

            {
                let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
                let mut mqtt_topic_handlers = mqtt_topic_handlers.write().await;
                mqtt_topic_handlers.clear()
            }
        }

        Ok(())
    }
}

/// The MQTT topic manager.
/// Subscribes and unsubscribes from topics.
pub struct MqttTopicManager<'a> {
    client: &'a mut Client,
    topics: Vec<Topic>,
}

impl<'a> MqttTopicManager<'a> {
    /// Initializes a new instance of the mqtt topic manager.
    fn new(client: &'a mut Client) -> Self {
        Self { client, topics: vec![] }
    }

    /// Add a new topic to the list.
    pub fn with_topic(mut self, topic: Topic) -> Self {
        self.topics.push(topic);
        self
    }

    /// Add a collection of topics to the list.
    pub fn with_topics(mut self, topics: Vec<Topic>) -> Self {
        self.topics.extend(topics.into_iter());
        self
    }

    /// Subscribe to the given topics with the callback.
    pub async fn subscribe<C: Fn(&crate::client::TopicEvent) + Send + Sync + 'static>(
        mut self,
        callback: C,
    ) -> Result<()> {
        let client = get_mqtt_client(&mut self.client).await?;
        let cb = Arc::new(Box::new(callback) as Box<dyn Fn(&crate::client::TopicEvent) + Send + Sync + 'static>);
        client
            .subscribe_many(
                self.topics
                    .iter()
                    .map(|t| SubscribeFilter::new(t.0.clone(), QoS::AtLeastOnce))
                    .collect::<Vec<SubscribeFilter>>(),
            )
            .await?;
        {
            let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
            let mut mqtt_topic_handlers = mqtt_topic_handlers.write().await;
            for topic in self.topics {
                match mqtt_topic_handlers.get_mut(&topic) {
                    Some(handlers) => handlers.push(cb.clone()),
                    None => {
                        mqtt_topic_handlers.insert(topic, vec![cb.clone()]);
                    }
                }
            }
        }
        Ok(())
    }

    /// Unsubscribe from the given topics.
    /// If no topics were provided, the function will unsubscribe from every subscribed topic.
    pub async fn unsubscribe(self) -> Result<()> {
        let topics = {
            let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
            let mqtt_topic_handlers = mqtt_topic_handlers.read().await;
            if self.topics.is_empty() {
                mqtt_topic_handlers.keys().cloned().collect()
            } else {
                self.topics
            }
        };

        if let Some(client) = &mut self.client.mqtt_client {
            for topic in topics.iter() {
                client.unsubscribe(&topic.0).await?;
            }
        }

        let empty_topic_handlers = {
            let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
            let mut mqtt_topic_handlers = mqtt_topic_handlers.write().await;
            for topic in topics {
                mqtt_topic_handlers.remove(&topic);
            }
            mqtt_topic_handlers.is_empty()
        };

        if self.client.broker_options.automatic_disconnect && empty_topic_handlers {
            MqttManager::new(self.client).disconnect().await?;
        }

        Ok(())
    }
}
