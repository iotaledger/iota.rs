// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use bee_message::prelude::*;
use bee_pow::providers::{MinerBuilder, ProviderBuilder};
use bee_signing_ext::{binary::BIP32Path, Seed};

use std::{num::NonZeroU64, str::FromStr};

const DEFAULT_NODE_URL: &str = "http://0.0.0.0:14265";

#[tokio::test]
async fn test_get_info() {
    let r = iota_client::Client::get_node_info(DEFAULT_NODE_URL).await.unwrap();
    println!("{:#?}", r);
}

#[tokio::test]
async fn test_get_health() {
    let r = iota_client::Client::get_node_health(DEFAULT_NODE_URL).await.unwrap();
    println!("{:#?}", r);
}

#[tokio::test]
async fn test_get_tips() {
    let r = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_tips()
        .await
        .unwrap();
    println!("{:#?}", r);
}

#[tokio::test]
async fn test_post_message_with_indexation() {
    let client = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap();

    let r = client
        .send_indexation_message()
        .index("Hello".to_string())
        .data("Tangle".to_string().as_bytes().to_vec())
        .post()
        .await
        .unwrap();

    println!("{}", r);
}

#[tokio::test]
async fn test_post_message_with_transaction() {
    let iota = iota_client::Client::builder() // Crate a client instance builder
        .node(DEFAULT_NODE_URL) // Insert the node here
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
        .send_transaction_message(&seed)
        .path(&path)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .output(
            "5eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8"
                .parse::<Ed25519Address>()
                .unwrap()
                .into(), // Insert the address to search for
            NonZeroU64::new(100).unwrap(),
        )
        .post()
        .await
        .unwrap();
    println!("Message ID: {:?}", message_id);
}

#[tokio::test]
async fn test_get_message_by_index() {
    let r = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .index("HORNET Spammer")
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[tokio::test]
async fn test_get_message_data() {
    let client = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap();

    let message_id = client
        .send_indexation_message()
        .index("Hello".to_string())
        .data("Tangle".to_string().as_bytes().to_vec())
        .post()
        .await
        .unwrap();

    let r = client.get_message().data(&message_id).await.unwrap();

    println!("{:#?}", r);
}

#[tokio::test]
async fn test_get_message_metadata() {
    let client = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap();

    let message_id = client
        .send_indexation_message()
        .index("Hello".to_string())
        .data("Tangle".to_string().as_bytes().to_vec())
        .post()
        .await
        .unwrap();

    let r = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .metadata(&message_id)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[tokio::test]
async fn test_get_message_raw() {
    iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .raw(&MessageId::from_str("637c93cc8c32400d157473a3db9db9c7f463f46374483b9dcc4ee35ce6957211").unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_message_children() {
    iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .children(&MessageId::from_str("a008ce3354591950232c0dacdfcb17c4f6457c5bf407eff1befaab5fa7b3b7b3").unwrap())
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_address_balance() {
    let r = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_address()
        .balance(
            &("6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92"
                .parse::<Ed25519Address>()
                .unwrap())
            .into(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[tokio::test]
async fn test_get_address_outputs() {
    let r = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_address()
        .outputs(
            &("d2adf03c21269b25a0bb4319471213161f2a4fb57b16cc2e505b87b2ca52d37d"
                .parse::<Ed25519Address>()
                .unwrap())
            .into(), // Insert the address to search for
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[tokio::test]
async fn test_get_output() {
    let r = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
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

#[tokio::test]
async fn test_get_milestone() {
    let r = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_milestone(50265)
        .await
        .unwrap();

    println!("{:#?}", r);
}
