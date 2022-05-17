// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

use std::fmt::{Debug, Display};

use bee_message::{output::NativeTokens, semantic::ConflictReason};
use serde::{ser::Serializer, Serialize};

use crate::node_api::indexer::QueryParameter;

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Serialize)]
/// Error type of the iota client crate.
#[allow(clippy::large_enum_variant)]
#[serde(tag = "type", content = "error")]
pub enum Error {
    /// Error when building tagged_data messages
    #[error("Error when building tagged_data message: {0}")]
    TaggedDataError(String),
    /// Invalid amount in API response
    #[error("Invalid amount in API response: {0}")]
    InvalidAmount(String),
    /// Error when building transaction messages
    #[error("Error when building transaction message")]
    TransactionError,
    /// The wallet account doesn't have enough balance
    #[error("The wallet account doesn't have enough balance. It only has {0}, required is {1}")]
    NotEnoughBalance(u64, u64),
    /// The wallet account doesn't have any inputs found
    #[error("No inputs found")]
    NoInputs,
    /// The wallet account doesn't have enough native tokens
    #[error("The wallet account doesn't have enough native tokens, missing: {0:?}")]
    NotEnoughNativeTokens(NativeTokens),
    // todo get missing amount (storage deposit for an output with this amount of native tokens)
    /// The wallet account doesn't have enough balance for an output with the remaining native tokens.
    #[error("The wallet account doesn't have enough balance for an output with the remaining native tokens.")]
    NoBalanceForNativeTokenRemainder,
    /// The wallet account has enough funds, but splitted on too many outputs
    #[error(
        "The wallet account has enough funds, but splitted on too many outputs: {0}, max. is 128, consolidate them"
    )]
    ConsolidationRequired(usize),
    /// Missing required parameters
    #[error("Must provide required parameter: {0}")]
    MissingParameter(&'static str),
    /// PlaceholderSecretManager can't be used for address generation or signing
    #[error("PlaceholderSecretManager can't be used for address generation or signing")]
    PlaceholderSecretManager,
    /// Invalid parameters
    #[error("Parameter is invalid:{0}")]
    InvalidParameter(&'static str),
    /// No node available in the synced node pool
    #[error("No synced node available")]
    SyncedNodePoolEmpty,
    /// Error on reaching quorum
    #[error("Failed to reach quorum {0} {1}")]
    QuorumThresholdError(usize, usize),
    /// Error on quorum because not enough nodes are available
    #[error("Not enough nodes for quorum {0} {1}")]
    QuorumPoolSizeError(usize, usize),
    /// Error on API request
    #[error("Node error: {0}")]
    NodeError(String),
    /// Hex string convert error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    FromHexError(#[from] hex::FromHexError),
    /// Logger error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    LoggerError(#[from] fern_logger::Error),
    /// Message types error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    MessageError(#[from] bee_message::Error),
    /// Message dtos error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    MessageDtoError(#[from] bee_message::DtoError),
    /// Bee rest api error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    BeeRestApiError(#[from] bee_rest_api::types::error::Error),
    /// The message doensn't need to be promoted or reattached
    #[error("Message ID `{0}` doesn't need to be promoted or reattached")]
    NoNeedPromoteOrReattach(String),
    /// The message cannot be included into the Tangle
    #[error("Message ID `{0}` couldn't get included into the Tangle")]
    TangleInclusionError(String),
    /// Mqtt client error
    #[cfg(feature = "mqtt")]
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    MqttClientError(#[from] rumqttc::ClientError),
    /// Invalid MQTT topic.
    #[error("The MQTT topic {0} is invalid")]
    InvalidMqttTopic(String),
    /// MQTT connection not found (all nodes MQTT's are disabled)
    #[error("MQTT connection not found (all nodes have the MQTT plugin disabled)")]
    MqttConnectionNotFound,
    /// IO error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    IoError(#[from] std::io::Error),
    /// JSON error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    Json(#[from] serde_json::Error),
    /// PoW error
    #[error("{0}")]
    Pow(String),
    /// Address not found
    #[error("Address: {0} not found in range: {1}")]
    InputAddressNotFound(String, String),
    /// Crypto.rs error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    CryptoError(#[from] crypto::Error),
    /// Error from RestAPI calls with unexpected status code response
    #[error("Response error with status code {0}: {1}, URL: {2}")]
    ResponseError(u16, String, String),
    /// reqwest error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    ReqwestError(#[from] reqwest::Error),
    /// URL error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    UrlError(#[from] url::ParseError),
    /// URL validation error
    #[error("{0}")]
    UrlValidationError(String),
    /// URL auth error
    #[error("Can't set {0} to URL")]
    UrlAuthError(String),
    /// Blake2b256 Error
    #[error("{0}")]
    Blake2b256Error(&'static str),
    /// Output Error
    #[error("Output error: {0}")]
    OutputError(&'static str),
    /// Not implemented, specially for the default impl of [crate::secret::SecretManager::signature_unlock()].
    #[error("No mnemonic was stored! Please implement signature_unlock() :)")]
    SignatureUnlockNotImplemented,
    #[cfg(not(target_family = "wasm"))]
    /// Tokio task join error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    TaskJoinError(#[from] tokio::task::JoinError),
    /// Invalid mnemonic error
    #[error("Invalid mnemonic {0}")]
    InvalidMnemonic(String),
    /// PoW error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    PowError(#[from] bee_pow::providers::miner::Error),
    /// Packable error
    #[error("Bee packable error")]
    PackableError,
    /// API error
    #[error("Invalid API name")]
    ApiError,
    /// Rw lock failed.
    #[error("Rw lock failed")]
    PoisonError,
    /// Specifically used for `TryInfo` implementations for `SecretManager`.
    #[error("cannot unwrap a SecretManager: type mismatch!")]
    SecretManagerMismatch,
    /// Missing unlock block error
    #[error("missing unlock block")]
    MissingUnlockBlock,
    /// No input with matching ed25519 unlock condition provided
    #[error("No input with matching ed25519 unlock condition provided")]
    MissingInputWithEd25519UnlockCondition,
    /// Ledger transport error
    #[cfg(feature = "ledger_nano")]
    #[error("ledger transport error")]
    LedgerMiscError,
    /// Dongle Locked
    #[cfg(feature = "ledger_nano")]
    #[error("ledger locked")]
    LedgerDongleLocked,
    /// Denied by User
    #[cfg(feature = "ledger_nano")]
    #[error("denied by user")]
    LedgerDeniedByUser,
    /// Ledger Device not found
    #[cfg(feature = "ledger_nano")]
    #[error("ledger device not found")]
    LedgerDeviceNotFound,
    /// Ledger Essence Too Large
    #[cfg(feature = "ledger_nano")]
    #[error("ledger essence too large")]
    LedgerEssenceTooLarge,
    /// Ledger transport error
    #[cfg(feature = "ledger_nano")]
    #[error("ledger app compiled for testnet but used with mainnet or vice versa")]
    LedgerNetMismatch,
    /// Wrong ledger seed error
    #[cfg(feature = "ledger_nano")]
    #[error("ledger mnemonic is mismatched")]
    LedgerMnemonicMismatch,
    /// Riker system error during Stronghold initialization
    #[cfg(feature = "stronghold")]
    #[error("Stronghold reported a system error: {0}")]
    #[serde(serialize_with = "display_string")]
    StrongholdActorSystemError(#[from] riker::system::SystemError),
    /// Procedure execution error from Stronghold
    #[cfg(feature = "stronghold")]
    #[error("Stronghold reported a procedure error: {0}")]
    StrongholdProcedureError(String),
    /// A mnemonic has been already stored into a Stronghold vault
    #[cfg(feature = "stronghold")]
    #[error("a mnemonic has already been stored in the Stronghold vault")]
    StrongholdMnemonicAlreadyStored,
    /// No password has been supplied to a Stronghold vault, or it has been cleared
    #[cfg(feature = "stronghold")]
    #[error("no password has been supplied, or the key has been cleared from the memory")]
    StrongholdKeyCleared,
    /// No snapshot path has been supplied
    #[cfg(feature = "stronghold")]
    #[error("no snapshot path has been supplied")]
    StrongholdSnapshotPathMissing,
    /// The semantic validation of a transaction failed.
    #[error("the semantic validation of a transaction failed")]
    TransactionSemantic(ConflictReason),
    /// Local time doesn't match the time of the latest milestone timestamp
    #[error("Local time {0} doesn't match the time of the latest milestone timestamp: {1}")]
    TimeNotSynced(u32, u32),
    /// An indexer API request contains a query parameter not supported by the endpoint.
    #[error("An indexer API request contains a query parameter not supported by the endpoint.")]
    UnsupportedQueryParameter(QueryParameter),
}

// map most errors to a single error but there are some errors that
// need special care.
// LedgerDongleLocked: Ask the user to unlock the dongle
// LedgerDeniedByUser: The user denied a signing
// LedgerDeviceNotFound: No usable Ledger device was found
// LedgerMiscError: Everything else.
// LedgerEssenceTooLarge: Essence with bip32 input indices need more space then the internal buffer is big
#[cfg(feature = "ledger_nano")]
impl From<iota_ledger::api::errors::APIError> for Error {
    fn from(error: iota_ledger::api::errors::APIError) -> Self {
        log::info!("ledger error: {}", error);
        match error {
            iota_ledger::api::errors::APIError::SecurityStatusNotSatisfied => Error::LedgerDongleLocked,
            iota_ledger::api::errors::APIError::ConditionsOfUseNotSatisfied => Error::LedgerDeniedByUser,
            iota_ledger::api::errors::APIError::TransportError => Error::LedgerDeviceNotFound,
            iota_ledger::api::errors::APIError::EssenceTooLarge => Error::LedgerEssenceTooLarge,
            _ => Error::LedgerMiscError,
        }
    }
}

/// Use this to serialize Error variants that implements Debug but not Serialize
fn display_string<T, S>(value: &T, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    T: Display,
    S: Serializer,
{
    value.to_string().serialize(serializer)
}
