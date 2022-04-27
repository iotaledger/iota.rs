// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example consolidation --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    api::consolidate_funds,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
};

/// In this example we will consolidate all funds in a range of addresses

#[tokio::main]
async fn main() -> Result<()> {
    let address_range = 0..150;
    // Create a client instance
    let client = Client::builder().with_node("http://localhost:14265")?.finish().await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();

    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_hex_seed(
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap(),
    )?);

    // Here all funds will be send to the address with the lowest index in the range
    let address = consolidate_funds(&client, &secret_manager, 0, address_range).await?;

    println!("Funds consolidated to {}", address);
    Ok(())
}
