// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example quorum --release
//! In this example we will get outputs with quorum, which will compare the responses from the nodes.

use iota_client::{node_api::indexer::query_parameters::QueryParameter, secret::SecretManager, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    let client = Client::builder()
        .with_node("https://api.testnet.shimmer.network")?
        .with_node(&node_url)?
        .with_node("http://localhost:14265")?
        .with_quorum(true)
        .with_min_quorum_size(3)
        .with_quorum_threshold(66)
        .finish()?;

    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    // Generate the first address
    let addresses = client
        .get_addresses(&secret_manager)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await?;

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids_response = client
        .basic_output_ids(vec![
            QueryParameter::Address(addresses[0].clone()),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;
    println!("Address outputs: {output_ids_response:?}");

    Ok(())
}
