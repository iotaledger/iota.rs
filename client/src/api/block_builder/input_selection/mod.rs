// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

mod automatic;
mod core;
mod helpers;
mod manual;
mod sender_issuer;
mod utxo_chains;

pub(crate) use self::core::is_alias_transition;
pub use self::{
    core::{Burn, BurnDto, Error, InputSelection, Requirement, Selected},
    helpers::minimum_storage_deposit_basic_output,
};
