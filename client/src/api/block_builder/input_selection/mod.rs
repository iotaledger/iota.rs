// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

mod automatic;
mod core;
mod helpers;
mod manual;
mod sender_issuer;
mod utxo_chains;

pub use self::core::{Burn, InputSelection, Requirement, Selected};
