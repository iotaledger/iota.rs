// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

/// MQTT related errors.
#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    /// Client error.
    #[error("client error {0}")]
    Client(#[from] rumqttc::ClientError),
    /// Connection not found, all nodes have their MQTT plugin disabled.
    #[error("connection not found, all nodes have their MQTT plugin disabled")]
    ConnectionNotFound,
    /// Crypto error.
    #[error("crypto error {0}")]
    Crypto(#[from] crypto::Error),
    /// Invalid topic.
    #[error("invalid topic {0}")]
    InvalidTopic(String),
}
