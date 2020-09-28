//! Error handling in iota-client crate.

use std::fmt;

/// Type alias of `Result` in iota-client
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
/// Error type of the iota client crate.
pub enum Error {
    /// Error when building transaction messages
    TransactionError,
    /// Missing required parameters
    MissingParameter,
    /// Invalid parameters
    InvalidParameter(String),
    /// Found Spent Address that still has balance
    SpentAddress,
    /// Missing required iota seed
    MissingNode,
    /// No node available in the node pool
    NodePoolEmpty,
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
    /// Error on Url type conversion
    UrlError,
    /// The wallet account doesn't have enough balance
    NotEnoughBalance(u64),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TransactionError => write!(f, "Error when building transaction message"),
            Error::MissingParameter => write!(f, "Must provide required parameters"),
            Error::InvalidParameter(s) => write!(f, "Parameter is invalid:{}", s),
            Error::SpentAddress => "Found Spent Address that still has balance.".fmt(f),
            Error::MissingNode => "Must provide node to create instance".fmt(f),
            Error::NodePoolEmpty => "No node available".fmt(f),
            Error::QuorumError => "Fail to find quorum result".fmt(f),
            Error::QuorumThreshold => "Quorum result didn't pass the minimum threshold".fmt(f),
            Error::ReqwestError(e) => e.fmt(f),
            Error::ResponseError(s) => s.fmt(f),
            Error::TernaryError => "Fail to convert message to trytes".fmt(f),
            Error::UrlError => "Fail to parse url".fmt(f),
            Error::NotEnoughBalance(v) => write!(
                f,
                "The wallet account doesn't have enough balance. It only has {:?}",
                v
            ),
        }
    }
}

impl std::error::Error for Error {}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::ReqwestError(error)
    }
}
