// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::{
    client::{BrokerOptions, Client, MqttEvent, TopicEvent, TopicHandlerMap},
    Result,
};
use bee_common::packable::Packable;
use bee_message::Message;
use crypto::utils;
use log::warn;
use regex::Regex;
use rumqttc::{
    AsyncClient as MqttClient, Event, EventLoop, Incoming, MqttOptions, QoS, Request, Subscribe, SubscribeFilter,
    Transport,
};
use tokio::sync::{watch::Sender, RwLock};

use std::{convert::TryFrom, sync::Arc, time::Instant};

/// A MQTT topic.
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
        let mut name: String = name.into();
        // Convert non hex index to hex
        let indexation_beginning = "messages/indexation/";
        if name.len() > indexation_beginning.len()
            && &name[0..indexation_beginning.len()] == indexation_beginning
            && hex::decode(&name[indexation_beginning.len()..name.len()]).is_err()
        {
            name = format!(
                "messages/indexation/{}",
                hex::encode(&name[indexation_beginning.len()..name.len()])
            );
        }

        let valid_topics = lazy_static!(
          ["milestones/latest", "milestones/confirmed", "messages", "messages/referenced"].to_vec() => Vec<&str>
        );
        let regexes = lazy_static!(
          [
            Regex::new(r"messages/([A-Fa-f0-9]{64})/metadata").expect("regex failed"),
            Regex::new(r"outputs/([A-Fa-f0-9]{64})(\d{4})").expect("regex failed"),
            // BIP-173 compliant bech32 address
            Regex::new("addresses/[\x21-\x7E]{1,30}1[A-Za-z0-9]+/outputs").expect("regex failed"),
            // ED25519 address hex
            Regex::new("addresses/ed25519/([A-Fa-f0-9]{64})/outputs").expect("regex failed"),
            Regex::new(r"messages/indexation/([a-f0-9]{2,128})").expect("regex failed"),
            Regex::new(r"transactions/([A-Fa-f0-9]{64})/included-message").expect("regex failed"),
          ].to_vec() => Vec<Regex>
        );

        if valid_topics.iter().any(|valid| valid == &name) || regexes.iter().any(|re| re.is_match(&name)) {
            let topic = Self(name);
            Ok(topic)
        } else {
            Err(crate::Error::InvalidMqttTopic(name))
        }
    }
}

async fn get_mqtt_client(client: &mut Client) -> Result<&mut MqttClient> {
    // if the client was disconnected, we clear it so we can start over
    if *client.mqtt_event_receiver().borrow() == MqttEvent::Disconnected {
        client.mqtt_client = None;
    }
    match client.mqtt_client {
        Some(ref mut c) => Ok(c),
        None => {
            let nodes = if client.node_manager.sync {
                #[cfg(not(feature = "wasm"))]
                {
                    client
                        .node_manager
                        .synced_nodes
                        .read()
                        .map_or(client.node_manager.nodes.clone(), |synced_nodes| synced_nodes.clone())
                }
                #[cfg(feature = "wasm")]
                {
                    client.node_manager.nodes.clone()
                }
            } else {
                client.node_manager.nodes.clone()
            };
            for node in nodes.iter() {
                let host = node.url.host_str().expect("Can't get host from URL");
                let mut entropy = [0u8; 8];
                utils::rand::fill(&mut entropy)?;
                let id = format!("iotars{}", hex::encode(entropy));
                let port = client.broker_options.port;
                let mut uri = format!(
                    "{}://{}:{}/mqtt",
                    if node.url.scheme() == "https" { "wss" } else { "ws" },
                    host,
                    node.url.port_or_known_default().unwrap_or(port)
                );

                if !client.broker_options.use_ws {
                    uri = host.to_string();
                };
                let mut mqtt_options = MqttOptions::new(id, uri, port);
                if client.broker_options.use_ws {
                    mqtt_options.set_transport(Transport::ws());
                }
                mqtt_options.set_connection_timeout(client.broker_options.timeout.as_secs());
                let (_, mut connection) = MqttClient::new(mqtt_options.clone(), 10);
                // poll the event loop until we find a ConnAck event,
                // which means that the mqtt client is ready to be used on this host
                // if the event loop returns an error, we check the next node
                let mut got_ack = false;
                while let Ok(event) = connection.poll().await {
                    if let Event::Incoming(Incoming::ConnAck(_)) = event {
                        got_ack = true;
                        break;
                    }
                }

                // if we found a valid mqtt connection, loop it on a separate thread
                if got_ack {
                    let (mqtt_client, connection) = MqttClient::new(mqtt_options, 10);
                    client.mqtt_client.replace(mqtt_client);
                    poll_mqtt(
                        client.mqtt_topic_handlers.clone(),
                        client.broker_options.clone(),
                        client.mqtt_event_channel.0.clone(),
                        connection,
                    );
                }
            }
            client.mqtt_client.as_mut().ok_or(crate::Error::MqttConnectionNotFound)
        }
    }
}

fn poll_mqtt(
    mqtt_topic_handlers_guard: Arc<RwLock<TopicHandlerMap>>,
    options: BrokerOptions,
    event_sender: Arc<Sender<MqttEvent>>,
    mut event_loop: EventLoop,
) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime");
        runtime.block_on(async move {
            // rumqttc performs automatic reconnection since we keep running the event loop
            // but the subscriptions are lost on reconnection, so we need to resubscribe
            // the `is_subscribed` flag is set to false on event error, so the ConnAck event
            // can perform the resubscriptions and reset `is_subscribed` to true.
            // we need the flag since the first ConnAck must be ignored.
            let mut is_subscribed = true;
            let mut error_instant = Instant::now();
            let mut connection_failure_count = 0;
            let handle = event_loop.handle();
            loop {
                let event = event_loop.poll().await;
                let mqtt_topic_handlers_guard = mqtt_topic_handlers_guard.clone();
                match event {
                    Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                        let _ = event_sender.send(MqttEvent::Connected);
                        if !is_subscribed {
                            is_subscribed = true;
                            // resubscribe topics
                            let mqtt_topic_handlers = mqtt_topic_handlers_guard.read().await;
                            let topics = mqtt_topic_handlers
                                .keys()
                                .map(|t| SubscribeFilter::new(t.0.clone(), QoS::AtLeastOnce))
                                .collect::<Vec<SubscribeFilter>>();
                            if !topics.is_empty() {
                                let _ = handle.send(Request::Subscribe(Subscribe::new_many(topics))).await;
                            }
                        }
                    }
                    Ok(Event::Incoming(Incoming::Publish(p))) => {
                        let topic = p.topic.clone();
                        crate::async_runtime::spawn(async move {
                            let mqtt_topic_handlers = mqtt_topic_handlers_guard.read().await;
                            if let Some(handlers) = mqtt_topic_handlers.get(&Topic(topic.clone())) {
                                let event = {
                                    if topic.as_str() == "messages"
                                        || topic.contains("messages/indexation/")
                                        || topic.contains("transactions/")
                                    {
                                        let mut payload = &*p.payload;
                                        match Message::unpack(&mut payload) {
                                            Ok(iota_message) => match serde_json::to_string(&iota_message) {
                                                Ok(message) => Ok(TopicEvent {
                                                    topic,
                                                    payload: message,
                                                }),
                                                Err(e) => {
                                                    warn!("Parsing iota message failed: {0}", e);
                                                    Err(())
                                                }
                                            },
                                            Err(e) => {
                                                warn!("Message unpacking failed: {0}", e);
                                                Err(())
                                            }
                                        }
                                    } else {
                                        Ok(TopicEvent {
                                            topic,
                                            payload: String::from_utf8_lossy(&p.payload).to_string(),
                                        })
                                    }
                                };
                                if let Ok(event) = event {
                                    for handler in handlers {
                                        handler(&event)
                                    }
                                };
                            }
                        });
                    }
                    Err(_) => {
                        if error_instant.elapsed().as_secs() < 5 {
                            connection_failure_count += 1;
                        } else {
                            connection_failure_count = 1;
                        }
                        if connection_failure_count == options.max_reconnection_attempts {
                            let _ = event_sender.send(MqttEvent::Disconnected);
                            break;
                        }
                        error_instant = Instant::now();
                        is_subscribed = false;
                    }
                    _ => {}
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

            let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
            let mut mqtt_topic_handlers = mqtt_topic_handlers.write().await;
            mqtt_topic_handlers.clear()
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
    pub async fn subscribe<C: Fn(&crate::client::TopicEvent) + Send + Sync + 'static>(self, callback: C) -> Result<()> {
        let client = get_mqtt_client(self.client).await?;
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
