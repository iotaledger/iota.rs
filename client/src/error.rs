// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

use std::fmt::Debug;

use iota_types::block::semantic::ConflictReason;
use packable::error::UnexpectedEOF;
use serde::{
    ser::{SerializeMap, Serializer},
    Serialize,
};

use crate::{api::input_selection::Error as InputSelectionError, node_api::indexer::QueryParameter};

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

/// Error type of the iota client crate.
#[derive(Debug, thiserror::Error)]
#[allow(clippy::large_enum_variant)]
pub enum Error {
    /// Block dtos error
    #[error("{0}")]
    ApiTypes(#[from] iota_types::api::core::error::Error),
    /// Blake2b256 Error
    #[error("{0}")]
    Blake2b256(&'static str),
    /// Block dtos error
    #[error("{0}")]
    BlockDto(#[from] iota_types::block::DtoError),
    /// Block types error
    #[error("{0}")]
    Block(#[from] iota_types::block::Error),
    /// The wallet account has enough funds, but split on too many outputs
    #[error("the wallet account has enough funds, but split on too many outputs: {0}, max. is 128, consolidate them")]
    ConsolidationRequired(usize),
    /// Crypto.rs error
    #[error("{0}")]
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
    Json(#[from] serde_json::Error),
    /// Missing required parameters
    #[error("must provide required parameter: {0}")]
    MissingParameter(&'static str),
    /// Error on API request
    #[error("node error: {0}")]
    Node(String),
    /// The block doesn't need to be promoted or reattached
    #[error("block ID `{0}` doesn't need to be promoted or reattached")]
    NoNeedPromoteOrReattach(String),
    /// The requested data was not found.
    #[error("the requested data {0} was not found.")]
    NotFound(String),
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
    Unpack(#[from] packable::error::UnpackError<iota_types::block::Error, UnexpectedEOF>),
    /// URL auth error
    #[error("can't set {0} to URL")]
    UrlAuth(&'static str),
    /// URL error
    #[error("{0}")]
    Url(#[from] url::ParseError),
    /// URL validation error
    #[error("{0}")]
    UrlValidation(String),
    /// Input selection error.
    #[error("{0}")]
    InputSelection(#[from] InputSelectionError),

    /// Participation error
    #[cfg(feature = "participation")]
    #[cfg_attr(docsrs, doc(cfg(feature = "participation")))]
    #[error("{0}")]
    Participation(#[from] iota_types::api::plugins::participation::error::Error),

    //////////////////////////////////////////////////////////////////////
    // Ledger Nano
    //////////////////////////////////////////////////////////////////////
    /// Denied by User
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    #[error("denied by user")]
    LedgerDeniedByUser,
    /// Dongle Locked
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    #[error("ledger locked")]
    LedgerDongleLocked,
    /// Ledger Device not found
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    #[error("ledger device not found")]
    LedgerDeviceNotFound,
    /// Ledger Essence Too Large
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    #[error("ledger essence too large")]
    LedgerEssenceTooLarge,
    /// Ledger transport error
    #[cfg(feature = "ledger_nano")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ledger_nano")))]
    #[error("ledger transport error")]
    LedgerMiscError,

    /// MQTT error.
    #[cfg(feature = "mqtt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "mqtt")))]
    #[error("MQTT error {0}")]
    Mqtt(#[from] crate::node_api::mqtt::Error),

    //////////////////////////////////////////////////////////////////////
    // Stronghold
    //////////////////////////////////////////////////////////////////////
    /// Stronghold client error
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[error("stronghold client error: {0}")]
    StrongholdClient(#[from] iota_stronghold::ClientError),
    /// Invalid stronghold password.
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[error("invalid stronghold password")]
    StrongholdInvalidPassword,
    /// No password has been supplied to a Stronghold vault, or it has been cleared
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[error("no password has been supplied, or the key has been cleared from the memory")]
    StrongholdKeyCleared,
    /// Stronghold memory error
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[error("stronghold memory error: {0}")]
    StrongholdMemory(#[from] iota_stronghold::MemoryError),
    /// A mnemonic has been already stored into a Stronghold vault
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[error("a mnemonic has already been stored in the Stronghold vault")]
    StrongholdMnemonicAlreadyStored,
    /// No mnemonic has been stored into the Stronghold vault
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[error("no mnemonic has been stored into the Stronghold vault")]
    StrongholdMnemonicMissing,
    /// Procedure execution error from Stronghold
    #[cfg(feature = "stronghold")]
    #[cfg_attr(docsrs, doc(cfg(feature = "stronghold")))]
    #[error("Stronghold reported a procedure error: {0}")]
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

// Serialize type with Display error
impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_map(Some(2))?;
        let mut kind_dbg = format!("{self:?}");
        // Convert first char to lowercase
        if let Some(r) = kind_dbg.get_mut(0..1) {
            r.make_ascii_lowercase();
        }
        // Split by whitespace for struct variants and split by `(` for tuple variants
        // Safe to unwrap because kind_dbg is never an empty string
        let kind = kind_dbg.split([' ', '(']).next().unwrap();
        seq.serialize_entry("type", &kind)?;
        seq.serialize_entry("error", &self.to_string())?;
        seq.end()
    }
}
