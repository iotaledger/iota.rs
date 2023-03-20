// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! MQTT types

use std::{collections::HashMap, sync::Arc, time::Duration};

use iota_types::block::{
    payload::{milestone::ReceiptMilestoneOption, MilestonePayload},
    Block,
};
use regex::RegexSet;
use serde::{de::Error as _, Deserialize, Deserializer};
use serde_json::Value;

use super::Error;

type TopicHandler = Box<dyn Fn(&TopicEvent) + Send + Sync>;

pub(crate) type TopicHandlerMap = HashMap<Topic, Vec<Arc<TopicHandler>>>;

/// An event from a MQTT topic.

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct TopicEvent {
    /// the MQTT topic.
    pub topic: String,
    /// The MQTT event payload.
    pub payload: MqttPayload,
}

/// The payload of an `TopicEvent`.

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MqttPayload {
    /// In case it contains JSON.
    Json(Value),
    /// In case it contains a `Block` object.
    Block(Block),
    /// In case it contains a `Milestone` object.
    MilestonePayload(MilestonePayload),
    /// In case it contains a `Receipt` object.
    Receipt(ReceiptMilestoneOption),
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[must_use]
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
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize)]
pub struct Topic(String);

impl TryFrom<String> for Topic {
    type Error = Error;

    fn try_from(value: String) -> std::result::Result<Self, Self::Error> {
        Self::try_new(value)
    }
}

impl<'de> Deserialize<'de> for Topic {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        Self::try_from(s).map_err(|err| D::Error::custom(format!("{err}")))
    }
}

impl Topic {
    /// Creates a new topic and checks if it's valid.
    pub fn try_new(topic: impl Into<String>) -> Result<Self, Error> {
        let valid_topics = lazy_static!(
        RegexSet::new([
            // Milestone topics.
            r"^milestone-info/latest$",
            r"^milestone-info/confirmed$",
            r"^milestones$",
            // Block topics.
            r"^blocks$",
            r"^blocks/transaction$",
            r"^blocks/transaction/tagged-data$",
            r"^blocks/transaction/tagged-data/0x((?:[a-f0-9]{2}){1,64})$",
            r"^blocks/tagged-data$",
            r"^blocks/tagged-data/0x((?:[a-f0-9]{2}){1,64})$",
            r"^block-metadata/0x([a-f0-9]{64})$",
            r"^block-metadata/referenced$",
            // Transaction topics.
            r"^transactions/0x([a-f0-9]{64})/included-block$",
            // Output topics.
            r"^outputs/0x([a-f0-9]{64})(\d{4})$",
            r"^outputs/alias/0x([a-f0-9]{64})$",
            r"^outputs/nft/0x([a-f0-9]{64})$",
            r"^outputs/foundry/0x([a-f0-9]{76})$",
            r"^outputs/unlock/(\+|address|storage-return|expiration|state-controller|governor|immutable-alias)/[\x21-\x7E]{1,30}1[A-Za-z0-9]+(?:/spent)?$",
            // Receipt topics.
            r"^receipts$",
        ]).expect("cannot build regex set") => RegexSet);
        let topic = topic.into();

        if valid_topics.is_match(&topic) {
            Ok(Self(topic))
        } else {
            Err(Error::InvalidTopic(topic))
        }
    }

    /// Creates a new topic without checking if the given string represents a valid topic.
    pub fn new_unchecked(value: String) -> Self {
        Self(value)
    }

    /// Returns the topic.
    pub fn topic(&self) -> &str {
        &self.0
    }
}
