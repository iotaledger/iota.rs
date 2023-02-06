// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

use std::fmt::{Debug, Display};

use iota_types::block::{
    output::{ChainId, NativeTokens, OutputId, TokenId},
    semantic::ConflictReason,
};
use packable::error::UnexpectedEOF;
use primitive_types::U256;
use serde::{ser::Serializer, Serialize};

use crate::{api::input_selection::Requirement, node_api::indexer::QueryParameter};

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Serialize)]
/// Error type of the iota client crate.
#[allow(clippy::large_enum_variant)]
#[serde(tag = "type", content = "error", rename_all = "camelCase")]
pub enum Error {
    /// Block dtos error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    ApiTypes(#[from] iota_types::api::core::error::Error),
    /// Blake2b256 Error
    #[error("{0}")]
    Blake2b256(&'static str),
    /// Block dtos error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    BlockDto(#[from] iota_types::block::DtoError),
    /// Block types error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    Block(#[from] iota_types::block::Error),
    /// The wallet account has enough funds, but split on too many outputs
    #[error("the wallet account has enough funds, but split on too many outputs: {0}, max. is 128, consolidate them")]
    ConsolidationRequired(usize),
    /// Crypto.rs error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    Crypto(#[from] crypto::Error),
    /// Address not found
    #[error("address: {address} not found in range: {range}")]
    InputAddressNotFound {
        /// The address that was not found.
        address: String,
        /// The range in which the address was not found.
        range: String,
    },
    /// Invalid amount in API response
    #[error("invalid amount in API response: {0}")]
    InvalidAmount(String),
    /// Invalid BIP32 chain data
    #[error("invalid BIP32 chain data")]
    InvalidBIP32ChainData,
    /// Invalid mnemonic error
    #[error("invalid mnemonic {0}")]
    InvalidMnemonic(String),
    /// The transaction essence is too large
    #[error("the transaction essence is too large. Its length is {length}, max length is {max_length}")]
    InvalidRegularTransactionEssenceLength {
        /// The found length.
        length: usize,
        /// The max supported length.
        max_length: usize,
    },
    /// The transaction payload is too large
    #[error("the transaction payload is too large. Its length is {length}, max length is {max_length}")]
    InvalidTransactionPayloadLength {
        /// The found length.
        length: usize,
        /// The max length.
        max_length: usize,
    },
    /// JSON error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    Json(#[from] serde_json::Error),
    /// Missing input for utxo chain
    #[error("missing input: {0}")]
    MissingInput(String),
    /// Missing required parameters
    #[error("must provide required parameter: {0}")]
    MissingParameter(&'static str),
    /// No input with matching ed25519 address provided
    #[error("no input with matching ed25519 address provided")]
    MissingInputWithEd25519Address,
    /// Error on API request
    #[error("node error: {0}")]
    Node(String),
    /// The block doesn't need to be promoted or reattached
    #[error("block ID `{0}` doesn't need to be promoted or reattached")]
    NoNeedPromoteOrReattach(String),
    /// The requested data was not found.
    #[error("the requested data {0} was not found.")]
    NotFound(String),
    /// The wallet account doesn't have any inputs found
    #[error("no inputs found")]
    NoInputs,
    /// The wallet account doesn't have enough native tokens
    #[error("the wallet account doesn't have enough native tokens, missing: {0:?}")]
    NotEnoughNativeTokens(NativeTokens),
    /// The wallet account doesn't have enough balance for an output with the remaining native tokens.
    #[error("the wallet account doesn't have enough balance for an output with the remaining native tokens.")]
    NoBalanceForNativeTokenRemainder,
    /// Output Error
    #[error("output error: {0}")]
    Output(&'static str),
    /// PlaceholderSecretManager can't be used for address generation or signing
    #[error("placeholderSecretManager can't be used for address generation or signing")]
    PlaceholderSecretManager,
    /// Rw lock failed.
    #[error("rw lock failed")]
    PoisonError,
    /// PoW error
    #[error("{0}")]
    Pow(String),
    /// Prefix hex string convert error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    PrefixHex(#[from] prefix_hex::Error),
    /// Error on quorum because not enough nodes are available
    #[error("not enough nodes for quorum: {available_nodes} < {minimum_threshold}")]
    QuorumPoolSizeError {
        /// The number of nodes available for quorum.
        available_nodes: usize,
        /// The minimum quorum threshold.
        minimum_threshold: usize,
    },
    /// Error on reaching quorum
    #[error("failed to reach quorum: {quorum_size} < {minimum_threshold}")]
    QuorumThresholdError {
        /// The current quorum size.
        quorum_size: usize,
        /// The minimum quorum threshold.
        minimum_threshold: usize,
    },
    /// Error from RestAPI calls with unexpected status code response
    #[error("response error with status code {code}: {text}, URL: {url}")]
    ResponseError {
        /// The status code.
        code: u16,
        /// The text from the response.
        text: String,
        /// The url of the API.
        url: String,
    },
    /// reqwest error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    Reqwest(#[from] reqwest::Error),
    /// Specifically used for `TryInfo` implementations for `SecretManager`.
    #[error("cannot unwrap a SecretManager: type mismatch!")]
    SecretManagerMismatch,
    /// No node available in the healthy node pool
    #[error("no healthy node available")]
    HealthyNodePoolEmpty,
    /// Error when building tagged_data blocks
    #[error("error when building tagged_data block: {0}")]
    TaggedData(String),
    /// The block cannot be included into the Tangle
    #[error("block ID `{0}` couldn't get included into the Tangle")]
    TangleInclusion(String),
    #[cfg(not(target_family = "wasm"))]
    /// Tokio task join error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    TaskJoin(#[from] tokio::task::JoinError),
    /// Local time doesn't match the time of the latest milestone timestamp
    #[error(
        "local time {current_time} doesn't match the time of the latest milestone timestamp: {milestone_timestamp}"
    )]
    TimeNotSynced {
        /// The local time.
        current_time: u32,
        /// The timestamp of the latest milestone.
        milestone_timestamp: u32,
    },
    /// The semantic validation of a transaction failed.
    #[error("the semantic validation of a transaction failed with conflict reason: {} - {0:?}", *.0 as u8)]
    TransactionSemantic(ConflictReason),
    /// Unexpected API response error
    #[error("unexpected API response")]
    UnexpectedApiResponse,
    /// An indexer API request contains a query parameter not supported by the endpoint.
    #[error("an indexer API request contains a query parameter not supported by the endpoint: {0}.")]
    UnsupportedQueryParameter(QueryParameter),
    /// Unpack error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    Unpack(#[from] packable::error::UnpackError<iota_types::block::Error, UnexpectedEOF>),
    /// URL auth error
    #[error("can't set {0} to URL")]
    UrlAuth(&'static str),
    /// URL error
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    Url(#[from] url::ParseError),
    /// URL validation error
    #[error("{0}")]
    UrlValidation(String),

    //////////////////////////////////////////////////////////////////////
    // Input Selection
    //////////////////////////////////////////////////////////////////////
    /// Required input is forbidden.
    #[error("required input {0} is forbidden")]
    RequiredInputIsForbidden(OutputId),
    /// Required input is not available.
    #[error("required input {0} is not available")]
    RequiredInputIsNotAvailable(OutputId),
    /// Unfulfillable requirement.
    #[error("unfulfillable requirement {0:?}")]
    // TODO better name?
    UnfulfillableRequirement(Requirement),
    /// No available inputs were provided to input selection
    #[error("no available inputs provided")]
    NoAvailableInputsProvided,
    /// No outputs were provided to input selection
    #[error("no outputs provided")]
    NoOutputsProvided,
    /// Insufficient amount provided.
    #[error("insufficient amount: found {found}, required {required}")]
    InsufficientAmount {
        /// The amount found.
        found: u64,
        /// The required amount.
        required: u64,
    },
    /// Insufficient native token amount provided.
    #[error("insufficient native token amount: found {found}, required {required}")]
    InsufficientNativeTokenAmount {
        /// The token ID.
        token_id: TokenId,
        /// The amount found.
        found: U256,
        /// The required amount.
        required: U256,
    },
    /// Can't burn and transition an output at the same time.
    #[error("can't burn and transition an output at the same time, chain ID: {0}")]
    BurnAndTransition(ChainId),

    /// Participation error
    #[cfg(feature = "participation")]
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    Participation(#[from] iota_types::api::plugins::participation::error::Error),

    //////////////////////////////////////////////////////////////////////
    // Ledger Nano
    //////////////////////////////////////////////////////////////////////
    /// Denied by User
    #[cfg(feature = "ledger_nano")]
    #[error("denied by user")]
    LedgerDeniedByUser,
    /// Dongle Locked
    #[cfg(feature = "ledger_nano")]
    #[error("ledger locked")]
    LedgerDongleLocked,
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
    #[error("ledger transport error")]
    LedgerMiscError,

    //////////////////////////////////////////////////////////////////////
    // MQTT
    //////////////////////////////////////////////////////////////////////
    /// Invalid MQTT topic.
    #[cfg(feature = "mqtt")]
    #[error("the MQTT topic {0} is invalid")]
    InvalidMqttTopic(String),
    /// Mqtt client error
    #[cfg(feature = "mqtt")]
    #[error("{0}")]
    #[serde(serialize_with = "display_string")]
    MqttClient(#[from] rumqttc::ClientError),
    /// MQTT connection not found (all nodes MQTT's are disabled)
    #[cfg(feature = "mqtt")]
    #[error("mQTT connection not found (all nodes have the MQTT plugin disabled)")]
    MqttConnectionNotFound,

    //////////////////////////////////////////////////////////////////////
    // Stronghold
    //////////////////////////////////////////////////////////////////////
    /// Stronghold client error
    #[cfg(feature = "stronghold")]
    #[error("stronghold client error: {0}")]
    #[serde(serialize_with = "display_string")]
    StrongholdClient(#[from] iota_stronghold::ClientError),
    /// Invalid stronghold password.
    #[cfg(feature = "stronghold")]
    #[error("invalid stronghold password")]
    StrongholdInvalidPassword,
    /// No password has been supplied to a Stronghold vault, or it has been cleared
    #[cfg(feature = "stronghold")]
    #[error("no password has been supplied, or the key has been cleared from the memory")]
    StrongholdKeyCleared,
    /// Stronghold memory error
    #[cfg(feature = "stronghold")]
    #[error("stronghold memory error: {0}")]
    #[serde(serialize_with = "display_string")]
    StrongholdMemory(#[from] iota_stronghold::MemoryError),
    /// A mnemonic has been already stored into a Stronghold vault
    #[cfg(feature = "stronghold")]
    #[error("a mnemonic has already been stored in the Stronghold vault")]
    StrongholdMnemonicAlreadyStored,
    /// No mnemonic has been stored into the Stronghold vault
    #[cfg(feature = "stronghold")]
    #[error("no mnemonic has been stored into the Stronghold vault")]
    StrongholdMnemonicMissing,
    /// Procedure execution error from Stronghold
    #[cfg(feature = "stronghold")]
    #[error("Stronghold reported a procedure error: {0}")]
    #[serde(serialize_with = "display_string")]
    StrongholdProcedure(#[from] iota_stronghold::procedures::ProcedureError),
}

// map most errors to a single error but there are some errors that
// need special care.
// LedgerDongleLocked: Ask the user to unlock the dongle
// LedgerDeniedByUser: The user denied a signing
// LedgerDeviceNotFound: No usable Ledger device was found
// LedgerMiscError: Everything else.
// LedgerEssenceTooLarge: Essence with bip32 input indices need more space then the internal buffer is big
#[cfg(feature = "ledger_nano")]
impl From<iota_ledger_nano::api::errors::APIError> for Error {
    fn from(error: iota_ledger_nano::api::errors::APIError) -> Self {
        log::info!("ledger error: {}", error);
        match error {
            iota_ledger_nano::api::errors::APIError::ConditionsOfUseNotSatisfied => Self::LedgerDeniedByUser,
            iota_ledger_nano::api::errors::APIError::EssenceTooLarge => Self::LedgerEssenceTooLarge,
            iota_ledger_nano::api::errors::APIError::SecurityStatusNotSatisfied => Self::LedgerDongleLocked,
            iota_ledger_nano::api::errors::APIError::TransportError => Self::LedgerDeviceNotFound,
            _ => Self::LedgerMiscError,
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
