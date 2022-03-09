// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Error handling in iota-client crate.

use bee_message::output::TokenId;

use primitive_types::U256;
use serde::ser::{SerializeStruct, Serializer};
use std::collections::HashMap;

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
/// Error type of the iota client crate.
#[allow(clippy::large_enum_variant)]
pub enum Error {
    /// Error when building tagged_data messages
    #[error("Error when building tagged_data message: {0}")]
    TaggedDataError(String),
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
    NotEnoughNativeTokens(HashMap<TokenId, U256>),
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
    FromHexError(#[from] hex::FromHexError),
    /// Bee common logger error
    #[error("{0}")]
    CommonError(#[from] bee_common::logger::Error),
    /// Message types error
    #[error("{0}")]
    MessageError(#[from] bee_message::Error),
    /// Bee rest api error
    #[error("{0}")]
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
    MqttClientError(#[from] rumqttc::ClientError),
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
    #[error("Address: {0} not found in range: {1}")]
    InputAddressNotFound(String, String),
    /// Crypto.rs error
    #[error("{0}")]
    CryptoError(#[from] crypto::Error),
    /// ureq error
    #[cfg(feature = "sync")]
    #[error("{0}")]
    UreqError(#[from] ureq::Error),
    /// Error from RestAPI calls with unexpected status code response
    #[cfg(any(feature = "async", feature = "wasm"))]
    #[error("Response error with status code {0}: {1}, URL: {2}")]
    ResponseError(u16, String, String),
    /// reqwest error
    #[cfg(any(feature = "async", feature = "wasm"))]
    #[error("{0}")]
    ReqwestError(#[from] reqwest::Error),
    /// URL error
    #[error("{0}")]
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
    #[cfg(not(feature = "wasm"))]
    /// Tokio task join error
    #[error("{0}")]
    TaskJoinError(#[from] tokio::task::JoinError),
    /// Invalid mnemonic error
    #[error("Invalid mnemonic {0}")]
    InvalidMnemonic(String),
    /// PoW error
    #[error("{0}")]
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
    /// Missing unlock block error
    #[error("missing unlock block")]
    MissingUnlockBlock,
    /// No input with matching ed25519 unlock condition provided
    #[error("No input with matching ed25519 unlock condition provided")]
    MissingInputWithEd25519UnlockCondition,
    /// No mnemonic was stored, specially for the default impl of [crate::signing::Signer::store_mnemonic()].
    #[error("No mnemonic was stored! Please implement store_mnemonic() :)")]
    NoMnemonicWasStored,
    /// Ledger transport error
    #[cfg(feature = "ledger")]
    #[error("ledger transport error")]
    LedgerMiscError,
    /// Dongle Locked
    #[cfg(feature = "ledger")]
    #[error("ledger locked")]
    LedgerDongleLocked,
    /// Denied by User
    #[cfg(feature = "ledger")]
    #[error("denied by user")]
    LedgerDeniedByUser,
    /// Ledger Device not found
    #[cfg(feature = "ledger")]
    #[error("ledger device not found")]
    LedgerDeviceNotFound,
    /// Ledger Essence Too Large
    #[cfg(feature = "ledger")]
    #[error("ledger essence too large")]
    LedgerEssenceTooLarge,
    /// Ledger transport error
    #[cfg(feature = "ledger")]
    #[error("ledger app compiled for testnet but used with mainnet or vice versa")]
    LedgerNetMismatch,
    /// Wrong ledger seed error
    #[cfg(feature = "ledger")]
    #[error("ledger mnemonic is mismatched")]
    LedgerMnemonicMismatch,
    /// Riker system error during Stronghold initialization
    #[cfg(feature = "stronghold")]
    #[error("Stronghold reported a system error: {0}")]
    StrongholdActorSystemError(#[from] riker::system::SystemError),
    /// Procedure execution error from Stronghold
    #[cfg(feature = "stronghold")]
    #[error("Stronghold reported a procedure error: {0}")]
    StrongholdProcedureError(String),
    /// A mnemonic has been already stored into a Stronghold vault
    #[cfg(feature = "stronghold")]
    #[error("a mnemonic has already been stored in the Stronghold vault")]
    StrongholdMnemonicAlreadyStored,
}

// map most errors to a single error but there are some errors that
// need special care.
// LedgerDongleLocked: Ask the user to unlock the dongle
// LedgerDeniedByUser: The user denied a signing
// LedgerDeviceNotFound: No usable Ledger device was found
// LedgerMiscError: Everything else.
// LedgerEssenceTooLarge: Essence with bip32 input indices need more space then the internal buffer is big
#[cfg(feature = "ledger")]
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

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        fn serialize_variant<S: Serializer>(
            error: &Error,
            serializer: S,
            variant_name: &str,
        ) -> std::result::Result<S::Ok, S::Error> {
            let mut state = serializer.serialize_struct("Error", 2)?;
            state.serialize_field("type", variant_name)?;
            state.serialize_field("error", &error.to_string())?;
            state.end()
        }

        match self {
            Self::TaggedDataError(_) => serialize_variant(self, serializer, "TaggedDataError"),
            Self::TransactionError => serialize_variant(self, serializer, "TransactionError"),
            Self::NotEnoughBalance(..) => serialize_variant(self, serializer, "NotEnoughBalance"),
            Self::NoInputs => serialize_variant(self, serializer, "NoInputs"),
            Self::NotEnoughNativeTokens(_) => serialize_variant(self, serializer, "NotEnoughNativeTokens"),
            Self::NotEnoughBalanceForNativeTokenRemainder => serialize_variant(self, serializer, "NotEnoughBalanceForNativeTokenRemainder"),
            Self::ConsolidationRequired(_) => serialize_variant(self, serializer, "ConsolidationRequired"),
            Self::MissingParameter(_) => serialize_variant(self, serializer, "MissingParameter"),
            Self::InvalidParameter(_) => serialize_variant(self, serializer, "InvalidParameter"),
            Self::SyncedNodePoolEmpty => serialize_variant(self, serializer, "SyncedNodePoolEmpty"),
            Self::QuorumThresholdError(_, _) => serialize_variant(self, serializer, "QuorumThresholdError"),
            Self::QuorumPoolSizeError(_, _) => serialize_variant(self, serializer, "QuorumPoolSizeError"),
            Self::NodeError(_) => serialize_variant(self, serializer, "NodeError"),
            Self::FromHexError(_) => serialize_variant(self, serializer, "FromHexError"),
            Self::CommonError(_) => serialize_variant(self, serializer, "CommonError"),
            Self::MessageError(_) => serialize_variant(self, serializer, "MessageError"),
            Self::BeeRestApiError(_) => serialize_variant(self, serializer, "BeeRestApiError"),
            Self::NoNeedPromoteOrReattach(_) => serialize_variant(self, serializer, "NoNeedPromoteOrReattach"),
            Self::TangleInclusionError(_) => serialize_variant(self, serializer, "TangleInclusionError"),
            #[cfg(feature = "mqtt")]
            Self::MqttClientError(_) => serialize_variant(self, serializer, "MqttClientError"),
            Self::InvalidMqttTopic(_) => serialize_variant(self, serializer, "InvalidMqttTopic"),
            Self::MqttConnectionNotFound => serialize_variant(self, serializer, "MqttConnectionNotFound"),
            Self::IoError(_) => serialize_variant(self, serializer, "IoError"),
            Self::Json(_) => serialize_variant(self, serializer, "Json"),
            Self::Pow(_) => serialize_variant(self, serializer, "Pow"),
            Self::InputAddressNotFound(_, _) => serialize_variant(self, serializer, "InputAddressNotFound"),
            Self::CryptoError(_) => serialize_variant(self, serializer, "CryptoError"),
            #[cfg(feature = "sync")]
            Self::UreqError(_) => serialize_variant(self, serializer, "UreqError"),
            #[cfg(any(feature = "async", feature = "wasm"))]
            Self::ResponseError(_, _, _) => serialize_variant(self, serializer, "ResponseError"),
            #[cfg(any(feature = "async", feature = "wasm"))]
            Self::ReqwestError(_) => serialize_variant(self, serializer, "ReqwestError"),
            Self::UrlError(_) => serialize_variant(self, serializer, "UrlError"),
            Self::UrlValidationError(_) => serialize_variant(self, serializer, "UrlValidationError"),
            Self::UrlAuthError(_) => serialize_variant(self, serializer, "UrlAuthError"),
            Self::Blake2b256Error(_) => serialize_variant(self, serializer, "Blake2b256Error"),
            Self::OutputError(_) => serialize_variant(self, serializer, "OutputError"),
            #[cfg(not(feature = "wasm"))]
            Self::TaskJoinError(_) => serialize_variant(self, serializer, "TaskJoinError"),
            Self::InvalidMnemonic(_) => serialize_variant(self, serializer, "InvalidMnemonic"),
            Self::PowError(_) => serialize_variant(self, serializer, "PowError"),
            Self::PackableError => serialize_variant(self, serializer, "PackableError"),
            Self::ApiError => serialize_variant(self, serializer, "ApiError"),
            Self::PoisonError => serialize_variant(self, serializer, "PoisonError"),
            Self::MissingUnlockBlock => serialize_variant(self, serializer, "MissingUnlockBlock"),
            Self::MissingInputWithEd25519UnlockCondition => serialize_variant(self, serializer, "MissingInputWithEd25519UnlockCondition"),
            #[cfg(feature = "ledger")]
            Self::LedgerMiscError => serialize_variant(self, serializer, "LedgerMiscError"),
            #[cfg(feature = "ledger")]
            Self::LedgerDongleLocked => serialize_variant(self, serializer, "LedgerDongleLocked"),
            #[cfg(feature = "ledger")]
            Self::LedgerDeniedByUser => serialize_variant(self, serializer, "LedgerDeniedByUser"),
            #[cfg(feature = "ledger")]
            Self::LedgerDeviceNotFound => serialize_variant(self, serializer, "LedgerDeviceNotFound"),
            #[cfg(feature = "ledger")]
            Self::LedgerEssenceTooLarge => serialize_variant(self, serializer, "LedgerEssenceTooLarge"),
            #[cfg(feature = "ledger")]
            Self::LedgerNetMismatch => serialize_variant(self, serializer, "LedgerNetMismatch"),
            #[cfg(feature = "ledger")]
            Self::LedgerMnemonicMismatch => serialize_variant(self, serializer, "LedgerMnemonicMismatch"),
        }
    }
}
