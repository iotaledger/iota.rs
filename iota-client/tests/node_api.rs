// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use bee_message::prelude::*;
use bee_signing_ext::{binary::BIP32Path, Seed};

use std::{convert::TryInto, num::NonZeroU64, str::FromStr};

#[ignore]
#[tokio::test]
async fn test_get_info() {
    iota_client::Client::get_node_info("http://0.0.0.0:14265")
        .await
        .unwrap();
}

#[ignore]
#[tokio::test]
async fn test_get_health() {
    iota_client::Client::get_node_health("http://0.0.0.0:14265")
        .await
        .unwrap();
}

#[ignore]
#[tokio::test]
async fn test_get_tips() {
    let r = iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_tips()
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_post_message_with_indexation() {
    let index = Indexation::new(String::from("Hello"), &[]).unwrap();

    let client = iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap();

    let tips = client.get_tips().await.unwrap();

    let message = Message::builder()
        .with_network_id(0)
        .with_parent1(tips.0)
        .with_parent2(tips.1)
        .with_payload(Payload::Indexation(Box::new(index)))
        .finish()
        .unwrap();

    let r = client.post_message(&message).await.unwrap();

    println!("{}", r);
}

#[ignore]
#[tokio::test]
async fn test_post_message_with_transaction() {
    let iota = iota_client::Client::builder() // Crate a client instance builder
        .node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    let seed = Seed::from_ed25519_bytes(
        &hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap(),
    )
    .unwrap();

    // Insert your account path. Note that index must be hardened(like 0', 123').
    let path = BIP32Path::from_str("m/").unwrap();
    let message_id = iota
        .send(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            Ed25519Address::new(
                hex::decode("5eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
            NonZeroU64::new(100).unwrap(),
        )
        .post()
        .await
        .unwrap();
    println!("Message ID: {:?}", message_id);
}

#[ignore]
#[tokio::test]
async fn test_get_message_by_index() {
    let r = iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .index("HORNET Spammer")
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_data() {
    let r = iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .data(&MessageId::from_str("1bf33857f8a3960b23d841fbf4a8b72b7bcb80e749d05abd95b85bcca816b600").unwrap())
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_metadata() {
    let r = iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .metadata(&MessageId::from_str("dc9492aaf06d12fd3927a3ce6e5e278edce930e0fa13ec3a09148ace6fe9448a").unwrap())
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_raw() {
    iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .raw(&MessageId::from_str("a008ce3354591950232c0dacdfcb17c4f6457c5bf407eff1befaab5fa7b3b7b3").unwrap())
        .await
        .unwrap();
}

#[ignore]
#[tokio::test]
async fn test_get_message_children() {
    iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .children(&MessageId::from_str("a008ce3354591950232c0dacdfcb17c4f6457c5bf407eff1befaab5fa7b3b7b3").unwrap())
        .await
        .unwrap();
}

#[ignore]
#[tokio::test]
async fn test_get_address_balance() {
    let r = iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_address()
        .balance(
            &Ed25519Address::new(
                hex::decode("6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_address_outputs() {
    let r = iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_address()
        .outputs(
            &Ed25519Address::new(
                hex::decode("d2adf03c21269b25a0bb4319471213161f2a4fb57b16cc2e505b87b2ca52d37d") // Insert the address to search for
                    .unwrap()
                    .try_into()
                    .unwrap(),
            )
            .into(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_output() {
    let r = iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_output(
            &UTXOInput::new(
                TransactionId::from_str("0000000000000000000000000000000000000000000000000000000000000000").unwrap(),
                0,
            )
            .unwrap(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_milestone() {
    let r = iota_client::Client::builder()
        .node("http://0.0.0.0:14265")
        .unwrap()
        .build()
        .unwrap()
        .get_milestone(2)
        .await
        .unwrap();

    println!("{:#?}", r);
}
