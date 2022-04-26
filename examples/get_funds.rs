// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example get_funds --release
use std::env;

use dotenv::dotenv;
use iota_client::{secret::mnemonic::MnemonicSecretManager, utils::request_funds_from_faucet, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let client = Client::builder()
        .with_node("http://localhost:14265") // Insert the node here
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let secret_manager =
        MnemonicSecretManager::try_from_mnemonic(&env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let addresses = client
        .get_addresses(&secret_manager)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();
    println!("{}", addresses[0]);

    let faucet_response =
        request_funds_from_faucet("http://localhost:14265/api/plugins/faucet/v1/enqueue", &addresses[0]).await?;

    println!("{}", faucet_response);
    Ok(())
}
