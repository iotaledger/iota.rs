// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.
use bee_message::{
    output::OutputId,
    payload::{transaction::TransactionId, Payload},
    MessageId,
};
use iota_client::{
    bech32_to_hex,
    node_api::indexer::query_parameters::QueryParameter,
    request_funds_from_faucet,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client,
};

const DEFAULT_DEVNET_NODE_URL: &str = "http://localhost:14265";
const DEFAULT_DEVNET_FAUCET_URL: &str = "http://localhost:14265";
// THIS SEED SERVES FOR TESTING PURPOSES! DON'T USE THIS SEED IN PRODUCTION!
const DEFAULT_DEVELOPMENT_SEED: &str = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";

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

async fn setup_secret_manager() -> SecretManager {
    SecretManager::Mnemonic(MnemonicSecretManager::try_from_hex_seed(DEFAULT_DEVELOPMENT_SEED).unwrap())
}

// Sends a transaction message to the node to test against it.
async fn setup_transaction_message() -> (MessageId, TransactionId) {
    let client = setup_client_with_sync_disabled().await;
    let secret_manager = setup_secret_manager().await;

    let addresses = client
        .get_addresses(&secret_manager)
        .with_range(0..2)
        .get_raw()
        .await
        .unwrap();
    println!(
        "{}",
        request_funds_from_faucet(
            DEFAULT_DEVNET_FAUCET_URL,
            &addresses[0].to_bech32(client.get_bech32_hrp().await.unwrap()),
        )
        .await
        .unwrap()
    );
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    let message_id = client
        .message()
        .with_secret_manager(&secret_manager)
        .with_output_hex(
            &bech32_to_hex(&addresses[1].to_bech32(client.get_bech32_hrp().await.unwrap())).unwrap(), /* Send funds
                                                                                                       * back to the
                                                                                                       * sender. */
            1_000_000, // The amount to spend, cannot be zero.
        )
        .unwrap()
        .finish()
        .await
        .unwrap()
        .id();

    let message = setup_client_with_sync_disabled()
        .await
        .get_message(&message_id)
        .await
        .unwrap();

    let transaction_id = match message.payload() {
        Some(Payload::Transaction(t)) => t.id(),
        _ => unreachable!(),
    };

    let _ = client.retry_until_included(&message.id(), None, None).await.unwrap();

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
    let r = client.get_message(&message_id).await.unwrap();

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
    let client = setup_client_with_sync_disabled().await;
    let secret_manager = setup_secret_manager().await;

    let address = client
        .get_addresses(&secret_manager)
        .with_range(0..1)
        .get_raw()
        .await
        .unwrap()[0];

    let r = client
        .get_address()
        .balance(&address.to_bech32(&client.get_bech32_hrp().await.unwrap()))
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_address_outputs() {
    let client = setup_client_with_sync_disabled().await;
    let secret_manager = setup_secret_manager().await;

    let address = client
        .get_addresses(&secret_manager)
        .with_range(0..1)
        .get_raw()
        .await
        .unwrap()[0];

    let address = client
        .output_ids(vec![QueryParameter::Address(
            address.to_bech32(&client.get_bech32_hrp().await.unwrap()),
        )])
        .await
        .unwrap();

    let r = client.get_outputs(address).await.unwrap();
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
async fn test_get_milestone_by_id() {
    let client = setup_client_with_sync_disabled().await;

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_milestone_by_id(node_info.nodeinfo.status.latest_milestone.milestone_id.parse().unwrap())
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_milestone_by_index() {
    let client = setup_client_with_sync_disabled().await;

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_milestone_by_index(node_info.nodeinfo.status.latest_milestone.index)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_utxo_changes_by_id() {
    let client = setup_client_with_sync_disabled().await;

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_utxo_changes_by_id(node_info.nodeinfo.status.latest_milestone.milestone_id.parse().unwrap())
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_utxo_changes_by_index() {
    let client = setup_client_with_sync_disabled().await;

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_utxo_changes_by_index(node_info.nodeinfo.status.latest_milestone.index)
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
