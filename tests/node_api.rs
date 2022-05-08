// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.
use bee_message::{
    output::OutputId,
    payload::{transaction::TransactionId, Payload},
    MessageId,
};
use iota_client::{
    node_api::indexer::query_parameters::QueryParameter,
    request_funds_from_faucet,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client,
};

const DEFAULT_DEVNET_NODE_URL: &str = "http://localhost:14265";
const DEFAULT_DEVNET_FAUCET_URL: &str = "http://localhost:14265";

// Sets up a Client with node synchronization disabled.
async fn setup_client_with_sync_disabled() -> Client {
    Client::builder()
        .with_node(DEFAULT_DEVNET_NODE_URL)
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap()
}

// Sends a tagged data message to the node to test against it.
async fn setup_tagged_data_message() -> MessageId {
    let client = setup_client_with_sync_disabled().await;

    client
        .message()
        .with_tag("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await
        .unwrap()
        .id()
}

// Sends a transaction message to the node to test against it.
async fn setup_transaction_message() -> (MessageId, TransactionId) {
    let client = setup_client_with_sync_disabled().await;

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    let secret_manager = SecretManager::Mnemonic(
        MnemonicSecretManager::try_from_hex_seed("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2")
            .unwrap(),
    );

    let address = client
        .get_addresses(&secret_manager)
        .with_range(0..1)
        .get_raw()
        .await
        .unwrap()[0];
    println!(
        "{}",
        request_funds_from_faucet(DEFAULT_DEVNET_FAUCET_URL, &address.to_bech32("rms"),)
            .await
            .unwrap()
    );
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    let message_id = client
        .message()
        .with_secret_manager(&secret_manager)
        // Insert the output address and ampunt to spent. The amount cannot be zero.
        .with_output_hex(
            "0x5eec99d6ee4ba21aa536c3364bbf2b587cb98a7f2565b75d948b10083e2143f8", // Insert the address to search for
            1_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap()
        .id();

    let message = setup_client_with_sync_disabled()
        .await
        .get_message_data(&message_id)
        .await
        .unwrap();

    let transaction_id = match message.payload() {
        Some(Payload::Transaction(t)) => t.id(),
        _ => unreachable!(),
    };

    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    (message_id, transaction_id)
}

#[ignore]
#[tokio::test]
async fn test_get_health() {
    let r = Client::get_node_health(DEFAULT_DEVNET_NODE_URL).await.unwrap();
    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_info() {
    let r = Client::get_node_info(DEFAULT_DEVNET_NODE_URL, None).await.unwrap();
    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_tips() {
    let r = setup_client_with_sync_disabled().await.get_tips().await.unwrap();
    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_post_message_with_tagged_data() {
    let message_id = setup_tagged_data_message().await;
    println!("{}", message_id);
}

#[ignore]
#[tokio::test]
async fn test_post_message_with_transaction() {
    let message_id = setup_transaction_message().await;
    println!("Message ID: {:?}", message_id);
}

#[ignore]
#[tokio::test]
async fn test_get_message_data() {
    let client = setup_client_with_sync_disabled().await;

    let message_id = setup_tagged_data_message().await;
    let r = client.get_message_data(&message_id).await.unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_metadata() {
    let message_id = setup_tagged_data_message().await;

    let r = setup_client_with_sync_disabled()
        .await
        .get_message_metadata(&message_id)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_raw() {
    let message_id = setup_tagged_data_message().await;

    let r = setup_client_with_sync_disabled()
        .await
        .get_message_raw(&message_id)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_message_children() {
    let message_id = setup_tagged_data_message().await;
    let r = setup_client_with_sync_disabled()
        .await
        .get_message_children(&message_id)
        .await
        .unwrap();
    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_address_balance() {
    let r = setup_client_with_sync_disabled()
        .await
        .get_address()
        .balance("rms1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx4aaacx")
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_address_outputs() {
    let client = setup_client_with_sync_disabled().await;
    let output_ids = client
        .output_ids(vec![QueryParameter::Address(
            "rms1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx4aaacx".to_string(),
        )])
        .await
        .unwrap();

    let r = client.get_outputs(output_ids).await.unwrap();
    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_output() {
    let (_message_id, transaction_id) = setup_transaction_message().await;

    let r = setup_client_with_sync_disabled()
        .await
        .get_output(&OutputId::new(transaction_id, 0).unwrap())
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_peers() {
    let r = setup_client_with_sync_disabled().await.get_peers().await.unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_milestone_by_milestone_id() {
    let client = setup_client_with_sync_disabled().await;

    // get node info first, because if we hardcode the milestones get pruned and if we hardcode an index it would fail
    // after some time
    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_milestone_by_milestone_id(node_info.nodeinfo.status.latest_milestone.milestone_id.parse().unwrap())
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_milestone_by_milestone_index() {
    let client = setup_client_with_sync_disabled().await;

    // get node info first, because if we hardcode the milestones get pruned and if we hardcode an index it would fail
    // after some time
    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_milestone_by_milestone_index(node_info.nodeinfo.status.latest_milestone.index)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_utxo_changes_by_milestone_id() {
    let client = setup_client_with_sync_disabled().await;

    // get node info first, because if we hardcode the milestones get pruned and if we hardcode an index it would fail
    // after some time
    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_utxo_changes_by_milestone_id(node_info.nodeinfo.status.latest_milestone.milestone_id.parse().unwrap())
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_utxo_changes_by_milestone_index() {
    let client = setup_client_with_sync_disabled().await;

    // get node info first, because if we hardcode the milestones get pruned and if we hardcode an index it would fail
    // after some time
    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_utxo_changes_by_milestone_index(node_info.nodeinfo.status.latest_milestone.index)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_receipts() {
    let r = setup_client_with_sync_disabled().await.get_receipts().await.unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn get_receipts_migrated_at() {
    let r = setup_client_with_sync_disabled()
        .await
        .get_receipts_migrated_at(3)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_treasury() {
    let r = setup_client_with_sync_disabled().await.get_treasury().await.unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_included_message() {
    let (_message_id, transaction_id) = setup_transaction_message().await;

    let r = setup_client_with_sync_disabled()
        .await
        .get_included_message(&transaction_id)
        .await
        .unwrap();

    println!("{:#?}", r);
}
