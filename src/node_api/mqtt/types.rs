// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! MQTT types

use crate::Result;

use regex::Regex;

use std::{collections::HashMap, sync::Arc, time::Duration};

type TopicHandler = Box<dyn Fn(&TopicEvent) + Send + Sync>;

pub(crate) type TopicHandlerMap = HashMap<Topic, Vec<Arc<TopicHandler>>>;

/// An event from a MQTT topic.

#[derive(Debug, Clone, serde::Serialize)]
pub struct TopicEvent {
    /// the MQTT topic.
    pub topic: String,
    /// The MQTT event payload.
    pub payload: String,
}

/// Mqtt events.

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MqttEvent {
    /// Client was connected.
    Connected,
    /// Client was disconnected.
    Disconnected,
}

/// The MQTT broker options.

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BrokerOptions {
    #[serde(default = "default_broker_automatic_disconnect", rename = "automaticDisconnect")]
    pub(crate) automatic_disconnect: bool,
    #[serde(default = "default_broker_timeout")]
    pub(crate) timeout: Duration,
    #[serde(default = "default_broker_use_ws", rename = "useWs")]
    pub(crate) use_ws: bool,
    #[serde(default = "default_broker_port")]
    pub(crate) port: u16,
    #[serde(default = "default_max_reconnection_attempts", rename = "maxReconnectionAttempts")]
    pub(crate) max_reconnection_attempts: usize,
}

fn default_broker_automatic_disconnect() -> bool {
    true
}

fn default_broker_timeout() -> Duration {
    Duration::from_secs(30)
}

fn default_broker_use_ws() -> bool {
    true
}

fn default_broker_port() -> u16 {
    1883
}

fn default_max_reconnection_attempts() -> usize {
    0
}

impl Default for BrokerOptions {
    fn default() -> Self {
        Self {
            automatic_disconnect: default_broker_automatic_disconnect(),
            timeout: default_broker_timeout(),
            use_ws: default_broker_use_ws(),
            port: default_broker_port(),
            max_reconnection_attempts: default_max_reconnection_attempts(),
        }
    }
}

impl BrokerOptions {
    /// Creates the default broker options.
    pub fn new() -> Self {
        Default::default()
    }

    /// Whether the MQTT broker should be automatically disconnected when all topics are unsubscribed or not.
    pub fn automatic_disconnect(mut self, automatic_disconnect: bool) -> Self {
        self.automatic_disconnect = automatic_disconnect;
        self
    }

    /// Sets the timeout used for the MQTT operations.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the use_ws used for the MQTT operations.
    pub fn use_ws(mut self, use_ws: bool) -> Self {
        self.use_ws = use_ws;
        self
    }

    /// Sets the port used for the MQTT operations.
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Sets the maximum number of reconnection attempts. 0 is unlimited.
    pub fn max_reconnection_attempts(mut self, max_reconnection_attempts: usize) -> Self {
        self.max_reconnection_attempts = max_reconnection_attempts;
        self
    }
}

/// A MQTT topic.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Topic(pub String);

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
