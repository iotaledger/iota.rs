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
use crate::core::*;

/// The Client strcut to connect through IRI with API usage
pub mod client;
/// IRI responses are parsed into structs contained in this module
pub mod responses;
/// Arguments for IOTA IRI APIs
pub mod options {
    pub use crate::attach_to_tangle::AttachOptions;
    pub use crate::find_transactions::FindTransactionsOptions;
    pub use crate::get_balances::GetBalancesOptions;
    pub use crate::get_inclusion_states::GetInclusionStatesOptions;
    pub use crate::get_transactions_to_approve::GetTransactionsToApproveOptions;
}

pub use client::Client;
pub use attach_to_tangle::attach_to_tangle_local;
