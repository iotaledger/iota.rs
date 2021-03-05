// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Error type of the iota client crate.
#[allow(clippy::large_enum_variant)]
pub enum Error {
    /// Error when building indexation messages
    #[error("Error when building indexation message: {0}")]
    IndexationError(String),
    /// Error when building transaction messages
    #[error("Error when building transaction message")]
    TransactionError,
    /// The wallet account doesn't have enough balance
    #[error("The wallet account doesn't have enough balance. It only has {0}, required is {1}")]
    NotEnoughBalance(u64, u64),
    /// Dust error, for example not enough balance on an address
    #[error("Dust error: {0}")]
    DustError(String),
    /// Missing required parameters
    #[error("Must provide required parameter: {0}")]
    MissingParameter(String),
    /// Invalid parameters
    #[error("Parameter is invalid:{0}")]
    InvalidParameter(String),
    /// Found spent output
    #[error("Found spent output.")]
    SpentOutput,
    /// No node available in the synced node pool
    #[error("No synced node available")]
    SyncedNodePoolEmpty,
    /// Error on Url type conversion
    #[error("Failed to parse node_pool_urls")]
    NodePoolUrlsError,
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
    #[error("Address not found in range {0}, address: {1}")]
    InputAddressNotFound(String, String),
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
    /// Crypto.rs error
    #[error("{0}")]
    CryptoError(crypto::Error),
    /// Invalid amount of parents
    #[error("Invalid amount of parents, length must be in 1..=8")]
    InvalidParentsAmount,
    /// ureq error
    #[error("{0}")]
    UreqError(#[from] ureq::Error),
    /// URL error
    #[error("{0}")]
    UrlError(#[from] url::ParseError),
    /// URL auth error
    #[error("Can't set {0} to URL")]
    UrlAuthError(String),
    /// DTO error
    #[error("failed to convert data: {0}")]
    DtoError(String),
}

// can't use #[from] on bee_message::Error so manually converting it
impl From<bee_message::Error> for Error {
    fn from(error: bee_message::Error) -> Self {
        Error::MessageError(error)
    }
}

// can't use #[from] on crypto::Error so manually converting it
impl From<crypto::Error> for Error {
    fn from(error: crypto::Error) -> Self {
        Error::CryptoError(error)
    }
}
