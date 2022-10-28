// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.
use iota_client::{
    bech32_to_hex,
    node_api::indexer::query_parameters::QueryParameter,
    request_funds_from_faucet,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client,
};
use iota_types::block::{
    output::OutputId,
    payload::{transaction::TransactionId, Payload},
    BlockId,
};

const DEFAULT_DEVNET_NODE_URL: &str = "http://localhost:14265";
const DEFAULT_DEVNET_FAUCET_URL: &str = "http://localhost:14265";
// THIS SEED SERVES FOR TESTING PURPOSES! DON'T USE THIS SEED IN PRODUCTION!
const DEFAULT_DEVELOPMENT_SEED: &str = "256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2";

// Sets up a Client with node synchronization disabled.
fn setup_client_with_sync_disabled() -> Client {
    Client::builder()
        .with_node(DEFAULT_DEVNET_NODE_URL)
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .unwrap()
}

// Sends a tagged data block to the node to test against it.
async fn setup_tagged_data_block() -> BlockId {
    let client = setup_client_with_sync_disabled();

    client
        .block()
        .with_tag("Hello".as_bytes().to_vec())
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await
        .unwrap()
        .id()
}

fn setup_secret_manager() -> SecretManager {
    SecretManager::Mnemonic(MnemonicSecretManager::try_from_hex_seed(DEFAULT_DEVELOPMENT_SEED).unwrap())
}

// Sends a transaction block to the node to test against it.
async fn setup_transaction_block() -> (BlockId, TransactionId) {
    let client = setup_client_with_sync_disabled();
    let secret_manager = setup_secret_manager();

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

    let block_id = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_output_hex(
            // Send funds back to the sender.
            &bech32_to_hex(&addresses[1].to_bech32(client.get_bech32_hrp().await.unwrap())).unwrap(),
            // The amount to spend, cannot be zero.
            1_000_000,
        )
        .await
        .unwrap()
        .finish()
        .await
        .unwrap()
        .id();

    let block = setup_client_with_sync_disabled().get_block(&block_id).await.unwrap();

    let transaction_id = match block.payload() {
        Some(Payload::Transaction(t)) => t.id(),
        _ => unreachable!(),
    };

    let _ = client.retry_until_included(&block.id(), None, None).await.unwrap();

    (block_id, transaction_id)
}

#[ignore]
#[tokio::test]
async fn test_get_health() {
    let r = setup_client_with_sync_disabled()
        .get_health(DEFAULT_DEVNET_NODE_URL)
        .await
        .unwrap();
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
    let r = setup_client_with_sync_disabled().get_tips().await.unwrap();
    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_post_block_with_tagged_data() {
    let block_id = setup_tagged_data_block().await;
    println!("{}", block_id);
}

#[ignore]
#[tokio::test]
async fn test_post_block_with_transaction() {
    let block_id = setup_transaction_block().await;
    println!("Block ID: {:?}", block_id);
}

#[ignore]
#[tokio::test]
async fn test_get_block_data() {
    let client = setup_client_with_sync_disabled();

    let block_id = setup_tagged_data_block().await;
    let r = client.get_block(&block_id).await.unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_block_metadata() {
    let block_id = setup_tagged_data_block().await;

    let r = setup_client_with_sync_disabled()
        .get_block_metadata(&block_id)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_block_raw() {
    let block_id = setup_tagged_data_block().await;

    let r = setup_client_with_sync_disabled()
        .get_block_raw(&block_id)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_address_outputs() {
    let client = setup_client_with_sync_disabled();
    let secret_manager = setup_secret_manager();

    let address = client
        .get_addresses(&secret_manager)
        .with_range(0..1)
        .get_raw()
        .await
        .unwrap()[0];

    let address = client
        .basic_output_ids(vec![QueryParameter::Address(
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
    let (_block_id, transaction_id) = setup_transaction_block().await;

    let r = setup_client_with_sync_disabled()
        .get_output(&OutputId::new(transaction_id, 0).unwrap())
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_peers() {
    let r = setup_client_with_sync_disabled().get_peers().await.unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_milestone_by_id() {
    let client = setup_client_with_sync_disabled();

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_milestone_by_id(
            &node_info
                .node_info
                .status
                .latest_milestone
                .milestone_id
                .unwrap()
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_milestone_by_index() {
    let client = setup_client_with_sync_disabled();

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_milestone_by_index(node_info.node_info.status.latest_milestone.index)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_utxo_changes_by_id() {
    let client = setup_client_with_sync_disabled();

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_utxo_changes_by_id(
            &node_info
                .node_info
                .status
                .latest_milestone
                .milestone_id
                .unwrap()
                .parse()
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_utxo_changes_by_index() {
    let client = setup_client_with_sync_disabled();

    let node_info = client.get_info().await.unwrap();

    let r = client
        .get_utxo_changes_by_index(node_info.node_info.status.latest_milestone.index)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_receipts() {
    let r = setup_client_with_sync_disabled().get_receipts().await.unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn get_receipts_migrated_at() {
    let r = setup_client_with_sync_disabled()
        .get_receipts_migrated_at(3)
        .await
        .unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_treasury() {
    let r = setup_client_with_sync_disabled().get_treasury().await.unwrap();

    println!("{:#?}", r);
}

#[ignore]
#[tokio::test]
async fn test_get_included_block() {
    let (_block_id, transaction_id) = setup_transaction_block().await;

    let r = setup_client_with_sync_disabled()
        .get_included_block(&transaction_id)
        .await
        .unwrap();

    println!("{:#?}", r);
}
