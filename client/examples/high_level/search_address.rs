// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example search_address --release

use iota_client::{
    api::search_address,
    constants::IOTA_COIN_TYPE,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
};

/// In this example we will try to find the index and address type of an address

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()?;

    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let addresses = client
        .get_addresses(&secret_manager)
        .with_account_index(0)
        .with_range(9..10)
        .get_raw()
        .await?;

    println!("{:?}", addresses[0]);

    let res = search_address(
        &secret_manager,
        &client.get_bech32_hrp().await?,
        IOTA_COIN_TYPE,
        0,
        0..10,
        &addresses[0],
    )
    .await?;

    println!("Address index: {}\nIs internal address: {}", res.0, res.1);
    Ok(())
}
