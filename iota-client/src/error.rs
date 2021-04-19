//! Error handling in iota-client crate.

use std::fmt;

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
/// Error type of the iota client crate.
pub enum Error {
    /// Missing required iota seed
    MissingSeed,
    /// Missing required iota seed
    MissingNode,
    /// No node available in the node pool
    NodePoolEmpty,
    /// Hash is not tail of the bundle
    NotTailHash,
    /// Error when processing quorum data
    QuorumError,
    /// Quorum result didn't pass the minimum threshold
    QuorumThreshold,
    /// Response error from IRI query
    ResponseError(String),
    /// Ternary conversion error
    TernaryError,
    /// Inputs balance cannot satisfy threshold requirement
    ThresholdNotEnough,
    /// No balance found
    NoBalance,
    /// Error on Url type conversion
    UrlError,
    /// Error on IO
    IoError(std::io::Error),
    /// ChrysalisAddressError
    ChrysalisAddressError(String),
    /// Crypto.rs error
    CryptoError(crypto::Error),
    /// Crypto.rs error
    Slip10Error(slip10::Error),
    /// ureq error
    UreqError(ureq::Error),
    /// Migration error
    MigrationError(&'static str),
    /// Ternary b1t6 decode error
    TernaryDecodeError(bee_ternary::b1t6::DecodeError),
    /// Bee ternary error
    BeeTernaryError(bee_ternary::Error),
    /// Bundle miner error
    BundleMinerError(iota_bundle_miner::error::Error),
    /// Bee transaction error
    BeeTransactionError(bee_transaction::bundled::BundledTransactionError),
    /// Bee bundled transaction error
    BeeOutgoingBundleError(bee_transaction::bundled::OutgoingBundleBuilderError),
    /// Bee crypto error
    BeeCryptoError(String),
    /// Crypto ternary error
    CryptoTernaryError(crypto::keys::ternary::seed::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingSeed => "Must provide seed to prepare transfer".fmt(f),
            Error::MissingNode => "Must provide node to create instance".fmt(f),
            Error::NodePoolEmpty => "No node available".fmt(f),
            Error::NotTailHash => "Provided hash is not tail".fmt(f),
            Error::QuorumError => "Fail to find quorum result".fmt(f),
            Error::QuorumThreshold => "Quorum result didn't pass the minimum threshold".fmt(f),
            Error::IoError(e) => e.fmt(f),
            Error::ResponseError(s) => s.fmt(f),
            Error::ThresholdNotEnough => "Cannot find enough inputs to satisify threshold".fmt(f),
            Error::NoBalance => "Can't find an address with balance'".fmt(f),
            Error::TernaryError => "Fail to convert message to trytes".fmt(f),
            Error::UrlError => "Fail to parse url".fmt(f),
            Error::ChrysalisAddressError(s) => s.fmt(f),
            Error::CryptoError(e) => e.fmt(f),
            Error::Slip10Error(e) => e.fmt(f),
            Error::UreqError(e) => e.fmt(f),
            Error::MigrationError(e) => e.fmt(f),
            Error::TernaryDecodeError(e) => format!("{:?}", e).fmt(f),
            Error::BeeTernaryError(e) => e.fmt(f),
            Error::BundleMinerError(e) => e.fmt(f),
            Error::BeeTransactionError(e) => format!("{:?}", e).fmt(f),
            Error::BeeOutgoingBundleError(e) => format!("{:?}", e).fmt(f),
            Error::BeeCryptoError(e) => e.fmt(f),
            Error::CryptoTernaryError(e) => format!("{:?}", e).fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::IoError(error)
    }
}
impl From<crypto::Error> for Error {
    fn from(error: crypto::Error) -> Self {
        Error::CryptoError(error)
    }
}
impl From<slip10::Error> for Error {
    fn from(error: slip10::Error) -> Self {
        Error::Slip10Error(error)
    }
}

impl From<ureq::Error> for Error {
    fn from(error: ureq::Error) -> Self {
        Error::UreqError(error)
    }
}

impl From<bee_ternary::b1t6::DecodeError> for Error {
    fn from(error: bee_ternary::b1t6::DecodeError) -> Self {
        Error::TernaryDecodeError(error)
    }
}
impl From<bee_ternary::Error> for Error {
    fn from(error: bee_ternary::Error) -> Self {
        Error::BeeTernaryError(error)
    }
}
impl From<iota_bundle_miner::error::Error> for Error {
    fn from(error: iota_bundle_miner::error::Error) -> Self {
        Error::BundleMinerError(error)
    }
}
impl From<bee_transaction::bundled::BundledTransactionError> for Error {
    fn from(error: bee_transaction::bundled::BundledTransactionError) -> Self {
        Error::BeeTransactionError(error)
    }
}
impl From<bee_transaction::bundled::OutgoingBundleBuilderError> for Error {
    fn from(error: bee_transaction::bundled::OutgoingBundleBuilderError) -> Self {
        Error::BeeOutgoingBundleError(error)
    }
}
impl From<crypto::keys::ternary::seed::Error> for Error {
    fn from(error: crypto::keys::ternary::seed::Error) -> Self {
        Error::CryptoTernaryError(error)
    }
}
