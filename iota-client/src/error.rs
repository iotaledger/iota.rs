//! Error handling in iota-client crate.

use std::fmt;

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

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
