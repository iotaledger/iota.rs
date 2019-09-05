#![deny(unused_extern_crates)]
#![allow(dead_code)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

//! Provides access to the Iota Client API

#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate serde_json;

mod core;
mod extended;
use crate::core::*;
use crate::extended::*;

/// The Client strcut to connect through IRI with API usage
pub mod client;
/// Arguments for IOTA IRI APIs
pub mod options {
    pub use crate::attach_to_tangle::AttachOptions;
    pub use crate::find_transactions::FindTransactionsOptions;
    pub use crate::get_balances::GetBalancesOptions;
    pub use crate::get_inclusion_states::GetInclusionStatesOptions;
    pub use crate::get_inputs::GetInputsOptions;
    pub use crate::get_new_address::GetNewAddressOptions;
    pub use crate::get_transactions_to_approve::GetTransactionsToApproveOptions;
    pub use crate::prepare_transfers::PrepareTransfersOptions;
    pub use crate::send_transfers::SendTransferOptions;
    pub use crate::send_trytes::SendTrytesOptions;
}

pub use attach_to_tangle::attach_to_tangle_local;
pub use client::Client;
pub use get_new_address::new_address;

type Result<T> = ::std::result::Result<T, failure::Error>;
