// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Input selection for transactions

mod automatic;
mod helpers;
mod manual;
/// TODO No need to document, will be removed in the future.
pub mod new;
mod sender_issuer;
pub mod types;
mod utxo_chains;

use crate::secret::types::InputSigningData;
