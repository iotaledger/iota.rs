// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{Client, Ed25519Address, Seed};
use std::{num::NonZeroU64, time::Duration};
use tokio::time::delay_for;

/// In this example, we send 900 tokens to the following 3 locations, respectively
///
/// Address Index 0
///   output 0: 300 tokens d25a0fb1d36b9760e6a893877a5bd0c316aba4d2504264dceb79287421b6448c
///   output 1: 300 tokens b41dc1fec07761335d66dc5e810bb3191cfa940f6756bb69ec8451e3c061b449
///   output 2: 300 tokens 3726ee414e23d398477bfdfa885815615e1a57b9a7e29a884335cb54a2bbb764
///
///
/// These two addresses belong to seed "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b1"
/// Then we send 550 tokens from seed "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b1"
/// to address "6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1fff", and check the ledger
/// inclusion state, which should be "included".

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap(),
    )
    .unwrap();

    let message_id = iota
        .send()
        .transaction(&seed)
        .account_index(0)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .output(
            "d25a0fb1d36b9760e6a893877a5bd0c316aba4d2504264dceb79287421b6448c"
                .parse::<Ed25519Address>()
                .unwrap()
                .into(), // Insert the address to search for
            NonZeroU64::new(300).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);
    delay_for(Duration::from_millis(15000)).await;
    let message_id = iota
        .send()
        .transaction(&seed)
        .account_index(0)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .output(
            "b41dc1fec07761335d66dc5e810bb3191cfa940f6756bb69ec8451e3c061b449"
                .parse::<Ed25519Address>()
                .unwrap()
                .into(),
            NonZeroU64::new(300).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);

    delay_for(Duration::from_millis(15000)).await;

    let message_id = iota
        .send()
        .transaction(&seed)
        .account_index(0)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .output(
            "3726ee414e23d398477bfdfa885815615e1a57b9a7e29a884335cb54a2bbb764"
                .parse::<Ed25519Address>()
                .unwrap()
                .into(),
            NonZeroU64::new(300).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);
    delay_for(Duration::from_millis(15000)).await;

    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b1").unwrap(),
    )
    .unwrap(); // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.

    delay_for(Duration::from_millis(15000)).await;

    let message_id = iota
        .send()
        .transaction(&seed)
        .account_index(0)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .output(
            "6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1fff"
                .parse::<Ed25519Address>()
                .unwrap()
                .into(),
            NonZeroU64::new(550).unwrap(),
        )
        .post()
        .await;

    println!("{:#?}", message_id);
    delay_for(Duration::from_millis(15000)).await;
    let message_metadata = iota.get_message().metadata(&message_id.unwrap()).await;
    println!(
        "The ledgerInclusionState: {:?}",
        message_metadata.unwrap().ledger_inclusion_state
    );
}
