// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use bee_message::prelude::*;
use crypto::keys::slip10::Seed;

use std::str::FromStr;

const DEFAULT_NODE_URL: &str = "https://api.lb-0.h.chrysalis-devnet.iota.cafe";

// Sends a full message object to the node with already computed nonce. Serves as a test object.
async fn setup_indexation_message() -> MessageId {
    let client = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap();

    client
        .message()
        .with_index("iota.rs")
        .with_data("iota.rs".as_bytes().to_vec())
        .finish()
        .await
        .unwrap()
        .id()
        .0
}

const DEFAULT_NODE_POOL_URLS: &str = "https://giftiota.com/nodes.json";
#[tokio::test]
#[ignore]
async fn test_with_node_pool_urls() {
    let r = iota_client::Client::builder()
        .with_node_pool_urls(&[DEFAULT_NODE_POOL_URLS.into()])
        .await
        .unwrap()
        .finish()
        .await;
    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_info() {
    let r = iota_client::Client::get_node_info(DEFAULT_NODE_URL, None, None)
        .await
        .unwrap();
    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_health() {
    let r = iota_client::Client::get_node_health(DEFAULT_NODE_URL).await.unwrap();
    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_tips() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_tips()
        .await
        .unwrap();
    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_post_message_with_indexation() {
    let client = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap();

    let r = client
        .message()
        .with_index(b"Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await
        .unwrap();

    println!("{}", r.id().0);
}

#[tokio::test]
#[ignore]
async fn test_post_message_with_transaction() {
    let iota = iota_client::Client::builder() // Crate a client instance builder
        .with_node(DEFAULT_NODE_URL) // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    let seed =
        Seed::from_bytes(&hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap());

    let message_id = iota
        .message()
        .with_seed(&seed)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .with_output_hex(
            "5eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8", // Insert the address to search for
            1_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();
    println!("Message ID: {message_id:?}");
}

#[tokio::test]
#[ignore]
async fn test_get_message_by_index() {
    setup_indexation_message().await;
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_message()
        .index("iota.rs")
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_message_data() {
    let client = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap();
    let message_id = setup_indexation_message().await;
    let r = client.get_message().data(&message_id).await.unwrap();
    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_message_metadata() {
    let message_id = setup_indexation_message().await;

    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_message()
        .metadata(&message_id)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_message_raw() {
    let message_id = setup_indexation_message().await;
    iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
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
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
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
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_address()
        .balance("atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r")
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_address_outputs() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_address()
        .outputs(
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r",
            Default::default(), /* Insert the address to
                                 * search for */
        )
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_output() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_output(
            &UtxoInput::new(
                TransactionId::from_str("3e18e19045d0b44dd2be3c466d6fe419c09342bacdb587f2985f2e607a92e38e").unwrap(),
                0,
            )
            .unwrap(),
        )
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_peers() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_peers()
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_milestone() {
    let client = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap();
    // get nodeinfo first, because if we hardocde the milestones get pruned and if we hardcode an index it would fail
    // after some time
    let nodeinfo = client.get_info().await.unwrap();
    let r = client
        .get_milestone(nodeinfo.nodeinfo.latest_milestone_index)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_milestone_utxo_changes() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_milestone_utxo_changes(3)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_receipts() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_receipts()
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn get_receipts_migrated_at() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_receipts_migrated_at(3)
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_treasury() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_treasury()
        .await
        .unwrap();

    println!("{r:#?}");
}

#[tokio::test]
#[ignore]
async fn test_get_included_message() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_included_message(
            &TransactionId::from_str("3e18e19045d0b44dd2be3c466d6fe419c09342bacdb587f2985f2e607a92e38e").unwrap(),
        )
        .await
        .unwrap();

    println!("{r:#?}");
}
