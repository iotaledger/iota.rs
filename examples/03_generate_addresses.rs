// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 03_generate_addresses --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    api::GetAddressesBuilder,
    constants::SHIMMER_TESTNET_BECH32_HRP,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
};

/// In this example we will create addresses from a seed defined in .env

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let client = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();
    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap(),
    )?);

    // Generate addresses with default account index and range
    let addresses = client.get_addresses(&secret_manager).finish().await.unwrap();
    println!("List of generated public addresses:\n{:?}\n", addresses);

    // Generate addresses with custom account index and range
    let addresses = client
        .get_addresses(&secret_manager)
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await?;

    println!("List of generated public addresses:\n{:?}\n", addresses);

    // Generate public (false) & internal (true) addresses
    let addresses = client.get_addresses(&secret_manager).with_range(0..4).get_all().await?;
    println!("List of generated public and internal addresses:\n{:?}\n", addresses);

    // Generate public addresses offline with the bech32_hrp defined
    let addresses = GetAddressesBuilder::new(&secret_manager)
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await?;

    println!("List of offline generated public addresses:\n{:?}\n", addresses);
    Ok(())
}
