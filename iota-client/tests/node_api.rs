// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use bee_message::prelude::*;
use bee_signing_ext::{binary::BIP32Path, Seed};

use iota_client::MessageJson;
use std::{convert::TryInto, num::NonZeroU64, str::FromStr};

const DEFAULT_NODE_URL: &str = "http://0.0.0.0:14265";

// Sends a full message object to the node with already computed nonce. Serves as a test object.
async fn setup_indexation_message() -> MessageId {
    let client = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap();
    let data = r#"
    {
	    "networkId": "6530425480034647824",
	    "parent1MessageId": "2e071ee19dc58d250e0e084a1ac890a9769896cd4c5689fd7f202bfc6c8d574c",
	    "parent2MessageId": "4375fb2a9d6b0b5a6c529bde678f227192d409b75cf87f7245ceeed8ed611664",
	    "payload": {
		    "type": 2,
		    "index": "HORNET Spammer",
		    "data": "42696e61727920697320746865206675747572652e0a436f756e743a203030373730370a54696d657374616d703a20323032302d31322d31345431343a33363a33342b30313a30300a54697073656c656374696f6e3a2035c2b573"
	    },
	    "nonce": "36952"
    }"#;
    let message: Message = serde_json::from_str::<MessageJson>(data).unwrap().try_into().unwrap();
    client.post_message(&message).await.unwrap()
}

#[tokio::test]
#[ignore]
async fn test_get_info() {
    let r = iota_client::Client::get_node_info(DEFAULT_NODE_URL).await.unwrap();
    println!("{:#?}", r);
}

#[tokio::test]
#[ignore]
async fn test_get_health() {
    let r = iota_client::Client::get_node_health(DEFAULT_NODE_URL).await.unwrap();
    println!("{:#?}", r);
}

#[tokio::test]
#[ignore]
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
#[ignore]
async fn test_post_message_with_indexation() {
    let client = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap();

    let r = client
        .send()
        .indexation()
        .index("Hello".to_string())
        .data("Tangle".to_string().as_bytes().to_vec())
        .post()
        .await
        .unwrap();

    println!("{}", r);
}

#[tokio::test]
#[ignore]
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
        .send()
        .transaction(&seed)
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
#[ignore]
async fn test_get_message_by_index() {
    setup_indexation_message().await;
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
#[ignore]
async fn test_get_message_data() {
    let client = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap();
    let message_id = setup_indexation_message().await;
    let r = client.get_message().data(&message_id).await.unwrap();
    println!("{:#?}", r);
}

#[tokio::test]
#[ignore]
async fn test_get_message_metadata() {
    let client = iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap();

    let message_id = setup_indexation_message().await;

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
#[ignore]
async fn test_get_message_raw() {
    let message_id = setup_indexation_message().await;
    iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .raw(&message_id)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore]
async fn test_get_message_children() {
    let message_id = setup_indexation_message().await;
    iota_client::Client::builder()
        .node(DEFAULT_NODE_URL)
        .unwrap()
        .build()
        .unwrap()
        .get_message()
        .children(&message_id)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore]
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
#[ignore]
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
#[ignore]
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
#[ignore]
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
