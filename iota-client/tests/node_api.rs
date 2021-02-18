// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

use bee_message::prelude::*;
use iota_client::Seed;

use bee_rest_api::types::MessageDto;
use std::{convert::TryFrom, str::FromStr};

const DEFAULT_NODE_URL: &str = "http://0.0.0.0:14265";

// Sends a full message object to the node with already computed nonce. Serves as a test object.
async fn setup_indexation_message() -> MessageId {
    let client = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap();
    let data = r#"
    {
	    "networkId": "6530425480034647824",
	    "parentMessageIds": [
            "2e071ee19dc58d250e0e084a1ac890a9769896cd4c5689fd7f202bfc6c8d574c", 
            "4375fb2a9d6b0b5a6c529bde678f227192d409b75cf87f7245ceeed8ed611664"
        ],
	    "payload": {
		    "type": 2,
		    "index": "HORNET Spammer",
		    "data": "42696e61727920697320746865206675747572652e0a436f756e743a203030373730370a54696d657374616d703a20323032302d31322d31345431343a33363a33342b30313a30300a54697073656c656374696f6e3a2035c2b573"
	    },
	    "nonce": "36952"
    }"#;
    let message = Message::try_from(&serde_json::from_str::<MessageDto>(data).unwrap()).unwrap();
    client.post_message(&message).await.unwrap()
}

// Ignored as long as we don't have a realy node pool url, otherwise the tests fail
// const DEFAULT_NODE_POOL_URLS: &str = "https://nodes.iota.works/api/ssl/live";
// #[test]
// fn test_with_node_pool_urls() {
//     let r = iota_client::Client::builder()
//         .with_node_pool_urls(&[DEFAULT_NODE_POOL_URLS.into()])
//         .unwrap()
//         .finish();
//     println!("{:#?}", r);
// }

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
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
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
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap();

    let r = client
        .message()
        .with_index(b"Hello")
        .with_data("Tangle".to_string().as_bytes().to_vec())
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
        Seed::from_bytes(&hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap())
            .unwrap();

    let message_id = iota
        .message()
        .with_seed(&seed)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .with_output_hex(
            "5eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8", // Insert the address to search for
            100,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();
    println!("Message ID: {:?}", message_id);
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
        .index("HORNET Spammer")
        .await
        .unwrap();

    println!("{:#?}", r);
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
    println!("{:#?}", r);
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

    println!("{:#?}", r);
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
        .balance(&"iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4".into())
        .await
        .unwrap();

    println!("{:#?}", r);
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
            &"iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4".into(), /* Insert the address to
                                                                                        * search for */
        )
        .await
        .unwrap();

    println!("{:#?}", r);
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

    println!("{:#?}", r);
}

#[tokio::test]
#[ignore]
async fn test_get_milestone() {
    let r = iota_client::Client::builder()
        .with_node(DEFAULT_NODE_URL)
        .unwrap()
        .finish()
        .await
        .unwrap()
        .get_milestone(3)
        .await
        .unwrap();

    println!("{:#?}", r);
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

    println!("{:#?}", r);
}
