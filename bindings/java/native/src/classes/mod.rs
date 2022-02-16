// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod client;

pub use client::*;

pub mod address;
pub mod ed25519;
pub mod message;
pub mod prepared;
pub mod slip10;

use std::borrow::Borrow;

pub fn consolidate_funds(
    client: crate::full_node_api::Client,
    seed: &str,
    account_index: usize,
    address_range_low: usize,
    address_range_high: usize,
) -> crate::Result<String> {
    match crate::block_on(async {
        iota_client::api::consolidate_funds(
            client.borrow(),
            &iota_client::Seed::from_bytes(&hex::decode(seed)?),
            account_index,
            address_range_low..address_range_high,
        )
        .await
    }) {
        Ok(s) => Ok(s),
        Err(e) => Err(anyhow::anyhow!(e.to_string())),
    }
}
