// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Error type of the iota client crate.
pub enum Error {
    /// Error when building indexation messages
    #[error("Error when building indexation message: {0}")]
    IndexationError(String),
    /// Error when building transaction messages
    #[error("Error when building transaction message")]
    TransactionError,
    /// The wallet account doesn't have enough balance
    #[error("The wallet account doesn't have enough balance. It only has {0}")]
    NotEnoughBalance(u64),
    /// Missing required parameters
    #[error("Must provide required parameter: {0}")]
    MissingParameter(String),
    /// Invalid parameters
    #[error("Parameter is invalid:{0}")]
    InvalidParameter(String),
    /// Found Spent Address that still has balance
    #[error("Found Spent Address that still has balance.")]
    SpentAddress,
    /// Error from RestAPI calls with unexpected status code response
    #[error("Response error with status code {0}: {1}")]
    ResponseError(u16, String),
    /// No node available in the synced node pool
    #[error("No node available")]
    SyncedNodePoolEmpty,
    /// Error on Url type conversion
    #[error("Failed to parse url")]
    UrlError,
    /// Error on Url type conversion
    #[error("Failed to parse node_pool_urls")]
    NodePoolUrlsError,
    /// Errors from reqwest api call
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    /// Hex string convert error
    #[error("{0}")]
    FromHexError(#[from] hex::FromHexError),
    /// Message types error
    #[error("{0}")]
    MessageError(bee_message::Error),
    /// The message cannot be promoted or reattached
    #[error("Message ID `{0}` doesn't need to be promoted or reattached")]
    NoNeedPromoteOrReattach(String),
    /// Mqtt client error
    #[cfg(feature = "mqtt")]
    #[error("{0}")]
    MqttClientError(#[from] paho_mqtt::errors::Error),
    /// Invalid MQTT topic.
    #[error("The MQTT topic {0} is invalid")]
    InvalidMqttTopic(String),
    /// MQTT connection not found (all nodes MQTT's are disabled)
    #[error("MQTT connection not found (all nodes have the MQTT plugin disabled)")]
    MqttConnectionNotFound,
    /// IO error
    #[error("{0}")]
    IoError(#[from] std::io::Error),
    /// JSON error
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    /// PoW error
    #[error("{0}")]
    Pow(String),
    /// Address not found
    #[error("Address not found in range {0}")]
    InputAddressNotFound(String),
    /// Storage adapter not set_path
    #[cfg(feature = "storage")]
    #[error("Storage adapter not set {0}")]
    StorageAdapterNotSet(String),
    /// Storage error
    #[cfg(feature = "storage")]
    #[error("Storage error {0}")]
    Storage(String),
    /// Account not found error
    #[cfg(feature = "storage")]
    #[error("Account not found")]
    AccountNotFound,
}

// can't use #[from] on bee_message::Error so manually converting it
impl From<bee_message::Error> for Error {
    fn from(error: bee_message::Error) -> Self {
        Error::MessageError(error)
    }
}
