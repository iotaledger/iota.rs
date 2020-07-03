//! Error handling in iota-client crate.

use std::fmt;

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
/// Error type of the iota client crate.
pub enum Error {
    /// Missing required iota seed
    MissingSeed,
    /// No node available in the node pool
    NodePoolEmpty,
    /// Hash is not tail of the bundle
    NotTailHash,
    /// Error when processing quorum data
    QuorumError,
    /// Quorum result didn't pass the minimum threshold
    QuorumThreshold,
    /// Errors from reqwest api call
    ReqwestError(reqwest::Error),
    /// Response error from IRI query
    ResponseError(String),
    /// Ternary conversion error
    TernaryError,
    /// Inputs balance cannot satisfy threshold requirement
    ThresholdNotEnough,
    /// Error on Url type conversion
    UrlError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::MissingSeed => "Must provide seed to prepare transfer".fmt(f),
            Error::NodePoolEmpty => "No node available".fmt(f),
            Error::NotTailHash => "Provided hash is not tail".fmt(f),
            Error::QuorumError => "Fail to find quorum result".fmt(f),
            Error::QuorumThreshold => "Quorum result didn't pass the minimum threshold".fmt(f),
            Error::ReqwestError(e) => e.fmt(f),
            Error::ResponseError(s) => s.fmt(f),
            Error::ThresholdNotEnough => "Cannot find enough inputs to satisify threshold".fmt(f),
            Error::TernaryError => "Fail to convert message to trytes".fmt(f),
            Error::UrlError => "Fail to parse url".fmt(f),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}
