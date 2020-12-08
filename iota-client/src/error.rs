// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

use std::fmt;

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
/// Error type of the iota client crate.
pub enum Error {
    /// Error when building transaction messages
    TransactionError,
    /// The wallet account doesn't have enough balance
    NotEnoughBalance(u64),
    /// Missing required parameters
    MissingParameter(String),
    /// Invalid parameters
    InvalidParameter(String),
    /// Found Spent Address that still has balance
    SpentAddress,
    /// Error from RestAPI calls with status code other than 200
    ResponseError(u16),
    /// No node available in the synced node pool
    SyncedNodePoolEmpty,
    /// Error on Url type conversion
    UrlError,
    /// Errors from reqwest api call
    ReqwestError(reqwest::Error),
    /// Hex string convert error
    FromHexError(hex::FromHexError),
    /// Message types error
    MessageError(bee_message::Error),
    /// The message cannot be promoted or reattached
    NoNeedPromoteOrReattach(String),
    /// Mqtt client error
    MqttClientError(paho_mqtt::errors::Error),
    /// Invalid MQTT topic.
    InvalidMqttTopic(String),
    /// MQTT connection not found (all nodes MQTT's are disabled)
    MqttConnectionNotFound,
    /// IO error
    IoError(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TransactionError => write!(f, "Error when building transaction message"),
            Error::MissingParameter(s) => write!(f, "Must provide required parameter:{}", s),
            Error::InvalidParameter(s) => write!(f, "Parameter is invalid:{}", s),
            Error::SpentAddress => "Found Spent Address that still has balance.".fmt(f),
            Error::SyncedNodePoolEmpty => "No node available".fmt(f),
            Error::ReqwestError(e) => e.fmt(f),
            Error::UrlError => "Fail to parse url".fmt(f),
            Error::NotEnoughBalance(v) => {
                write!(f, "The wallet account doesn't have enough balance. It only has {:?}", v)
            }
            Error::FromHexError(e) => e.fmt(f),
            Error::ResponseError(s) => write!(f, "Response error with status code {}", s),
            Error::MessageError(e) => e.fmt(f),
            Error::NoNeedPromoteOrReattach(s) => {
                write!(f, "Message ID {} cannot be promoted or reattached", s)
            }
            Error::MqttClientError(e) => e.fmt(f),
            Error::InvalidMqttTopic(topic) => write!(f, "The topic {} is invalid", topic),
            Error::MqttConnectionNotFound => {
                write!(f, "MQTT connection not found (all nodes have the MQTT plugin disabled)")
            }
            Error::IoError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(error: hex::FromHexError) -> Self {
        Error::FromHexError(error)
    }
}

impl From<bee_message::Error> for Error {
    fn from(error: bee_message::Error) -> Self {
        Error::MessageError(error)
    }
}

impl From<paho_mqtt::errors::Error> for Error {
    fn from(error: paho_mqtt::errors::Error) -> Self {
        Error::MqttClientError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}
