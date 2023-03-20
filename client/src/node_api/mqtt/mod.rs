// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA node MQTT API

mod error;
pub mod types;

use std::{
    sync::{Arc, RwLock as StdRwLock},
    time::Instant,
};

use crypto::utils;
use iota_types::block::{
    payload::{milestone::ReceiptMilestoneOption, MilestonePayload},
    Block,
};
use log::warn;
use packable::PackableExt;
use rumqttc::{AsyncClient, Event, EventLoop, Incoming, MqttOptions, NetworkOptions, QoS, SubscribeFilter, Transport};
use tokio::sync::{
    watch::{Receiver as WatchReceiver, Sender},
    RwLock,
};

pub use self::{error::Error, types::*};
use crate::{Client, NetworkInfo};

impl Client {
    /// Returns a handle to the MQTT topics manager.
    pub fn subscriber(&self) -> MqttManager<'_> {
        MqttManager::new(self)
    }

    /// Subscribe to MQTT events with a callback.
    pub async fn subscribe<C: Fn(&TopicEvent) + Send + Sync + 'static>(
        &self,
        topics: Vec<Topic>,
        callback: C,
    ) -> Result<(), Error> {
        MqttManager::new(self).with_topics(topics).subscribe(callback).await
    }

    /// Unsubscribe from MQTT events.
    pub async fn unsubscribe(&self, topics: Vec<Topic>) -> Result<(), Error> {
        MqttManager::new(self).with_topics(topics).unsubscribe().await
    }

    /// Returns the mqtt event receiver.
    pub fn mqtt_event_receiver(&self) -> WatchReceiver<MqttEvent> {
        self.mqtt_event_channel.1.clone()
    }
}

async fn set_mqtt_client(client: &Client) -> Result<(), Error> {
    // if the client was disconnected, we clear it so we can start over
    if *client.mqtt_event_receiver().borrow() == MqttEvent::Disconnected {
        *client.mqtt_client.write().await = None;
    }
    let exists = client.mqtt_client.read().await.is_some();

    if !exists {
        let nodes = if !client.node_manager.ignore_node_health {
            #[cfg(not(target_family = "wasm"))]
            {
                client
                    .node_manager
                    .healthy_nodes
                    .read()
                    .map_or(client.node_manager.nodes.clone(), |healthy_nodes| {
                        healthy_nodes.iter().map(|(node, _)| node.clone()).collect()
                    })
            }
            #[cfg(target_family = "wasm")]
            {
                client.node_manager.nodes.clone()
            }
        } else {
            client.node_manager.nodes.clone()
        };
        for node in &nodes {
            let host = node.url.host_str().expect("can't get host from URL");
            let mut entropy = [0u8; 8];
            utils::rand::fill(&mut entropy)?;
            let id = format!("iotars{}", prefix_hex::encode(entropy));
            let port = client.broker_options.port;
            let mut uri = format!(
                "{}://{}:{}/api/mqtt/v1",
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
            let (_, mut connection) = AsyncClient::new(mqtt_options.clone(), 10);
            connection.set_network_options(
                *NetworkOptions::new().set_connection_timeout(client.broker_options.timeout.as_secs()),
            );
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
                let (mqtt_client, connection) = AsyncClient::new(mqtt_options, 10);
                client.mqtt_client.write().await.replace(mqtt_client.clone());
                poll_mqtt(
                    mqtt_client,
                    client.mqtt_topic_handlers.clone(),
                    client.broker_options.clone(),
                    client.mqtt_event_channel.0.clone(),
                    connection,
                    client.network_info.clone(),
                );
            }
        }
    }
    Ok(())
}

fn poll_mqtt(
    mqtt_client: AsyncClient,
    mqtt_topic_handlers_guard: Arc<RwLock<TopicHandlerMap>>,
    options: BrokerOptions,
    event_sender: Arc<Sender<MqttEvent>>,
    mut event_loop: EventLoop,
    network_info: Arc<StdRwLock<NetworkInfo>>,
) {
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("failed to create Tokio runtime");
        runtime.block_on(async move {
            // rumqttc performs automatic reconnection since we keep running the event loop
            // but the subscriptions are lost on reconnection, so we need to resubscribe
            // the `is_subscribed` flag is set to false on event error, so the ConnAck event
            // can perform the re-subscriptions and reset `is_subscribed` to true.
            // we need the flag since the first ConnAck must be ignored.
            let mut is_subscribed = true;
            let mut error_instant = Instant::now();
            let mut connection_failure_count = 0;

            loop {
                let event = event_loop.poll().await;
                let mqtt_topic_handlers_guard = mqtt_topic_handlers_guard.clone();

                match event {
                    Ok(Event::Incoming(Incoming::ConnAck(_))) => {
                        let _ = event_sender.send(MqttEvent::Connected);
                        if !is_subscribed {
                            is_subscribed = true;
                            // resubscribe topics
                            let topics = mqtt_topic_handlers_guard
                                .read()
                                .await
                                .keys()
                                .map(|t| SubscribeFilter::new(t.topic().to_string(), QoS::AtLeastOnce))
                                .collect::<Vec<SubscribeFilter>>();
                            if !topics.is_empty() {
                                let _ = mqtt_client.subscribe_many(topics).await;
                            }
                        }
                    }
                    Ok(Event::Incoming(Incoming::Publish(p))) => {
                        let topic = p.topic.clone();
                        let network_info = network_info.clone();

                        crate::async_runtime::spawn(async move {
                            let mqtt_topic_handlers = mqtt_topic_handlers_guard.read().await;

                            if let Some(handlers) = mqtt_topic_handlers.get(&Topic::new_unchecked(topic.clone())) {
                                let event = {
                                    if topic.contains("blocks") || topic.contains("included-block") {
                                        let payload = &*p.payload;
                                        let protocol_parameters = &network_info.read().unwrap().protocol_parameters;

                                        match Block::unpack_verified(payload, protocol_parameters) {
                                            Ok(block) => Ok(TopicEvent {
                                                topic,
                                                payload: MqttPayload::Block(block),
                                            }),
                                            Err(e) => {
                                                warn!("Block unpacking failed: {:?}", e);
                                                Err(())
                                            }
                                        }
                                    } else if topic.contains("milestones") {
                                        let payload = &*p.payload;
                                        let protocol_parameters = &network_info.read().unwrap().protocol_parameters;

                                        match MilestonePayload::unpack_verified(payload, protocol_parameters) {
                                            Ok(milestone_payload) => Ok(TopicEvent {
                                                topic,
                                                payload: MqttPayload::MilestonePayload(milestone_payload),
                                            }),
                                            Err(e) => {
                                                warn!("MilestonePayload unpacking failed: {:?}", e);
                                                Err(())
                                            }
                                        }
                                    } else if topic.contains("receipts") {
                                        let payload = &*p.payload;
                                        let protocol_parameters = &network_info.read().unwrap().protocol_parameters;

                                        match ReceiptMilestoneOption::unpack_verified(payload, protocol_parameters) {
                                            Ok(receipt) => Ok(TopicEvent {
                                                topic,
                                                payload: MqttPayload::Receipt(receipt),
                                            }),
                                            Err(e) => {
                                                warn!("Receipt unpacking failed: {:?}", e);
                                                Err(())
                                            }
                                        }
                                    } else {
                                        match serde_json::from_slice(&p.payload) {
                                            Ok(value) => Ok(TopicEvent {
                                                topic,
                                                payload: MqttPayload::Json(value),
                                            }),
                                            Err(e) => {
                                                warn!("Cannot parse JSON: {:?}", e);
                                                Err(())
                                            }
                                        }
                                    }
                                };
                                if let Ok(event) = event {
                                    for handler in handlers {
                                        handler(&event);
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
    client: &'a Client,
}

impl<'a> MqttManager<'a> {
    /// Initializes a new instance of the mqtt subscriber.
    pub fn new(client: &'a Client) -> Self {
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
    pub async fn unsubscribe(self) -> Result<(), Error> {
        MqttTopicManager::new(self.client).unsubscribe().await
    }

    /// Disconnects the broker.
    /// This will clear the stored topic handlers and close the MQTT connection.
    pub async fn disconnect(self) -> Result<(), Error> {
        if let Some(client) = &*self.client.mqtt_client.write().await {
            client.disconnect().await?;
            let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
            let mut mqtt_topic_handlers = mqtt_topic_handlers.write().await;
            mqtt_topic_handlers.clear();
        }

        *self.client.mqtt_client.write().await = None;

        Ok(())
    }
}

/// The MQTT topic manager.
/// Subscribes and unsubscribes from topics.
pub struct MqttTopicManager<'a> {
    client: &'a Client,
    topics: Vec<Topic>,
}

impl<'a> MqttTopicManager<'a> {
    /// Initializes a new instance of the mqtt topic manager.
    fn new(client: &'a Client) -> Self {
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
    pub async fn subscribe<C: Fn(&crate::node_api::mqtt::TopicEvent) + Send + Sync + 'static>(
        self,
        callback: C,
    ) -> Result<(), Error> {
        let cb =
            Arc::new(Box::new(callback) as Box<dyn Fn(&crate::node_api::mqtt::TopicEvent) + Send + Sync + 'static>);
        set_mqtt_client(self.client).await?;
        self.client
            .mqtt_client
            .write()
            .await
            .as_ref()
            .ok_or(Error::ConnectionNotFound)?
            .subscribe_many(
                self.topics
                    .iter()
                    .map(|t| SubscribeFilter::new(t.topic().to_string(), QoS::AtLeastOnce))
                    .collect::<Vec<SubscribeFilter>>(),
            )
            .await?;
        {
            let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
            let mut mqtt_topic_handlers = mqtt_topic_handlers.write().await;
            for topic in self.topics {
                #[allow(clippy::option_if_let_else)]
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
    pub async fn unsubscribe(self) -> Result<(), Error> {
        let topics = {
            let mqtt_topic_handlers = &self.client.mqtt_topic_handlers;
            let mqtt_topic_handlers = mqtt_topic_handlers.read().await;
            if self.topics.is_empty() {
                mqtt_topic_handlers.keys().cloned().collect()
            } else {
                self.topics
            }
        };

        if let Some(client) = &*self.client.mqtt_client.write().await {
            for topic in &topics {
                client.unsubscribe(topic.topic()).await?;
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
