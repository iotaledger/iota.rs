// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 09_transaction --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
};

/// In this example we will send a transaction

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();
    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap(),
    )?);

    let message = client
        .message()
        .with_secret_manager(&secret_manager)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            // We generate an address from our seed so that we send the funds to ourselves
            &client.get_addresses(&secret_manager).with_range(1..2).finish().await?[0],
            1_000_000,
        )?
        .finish()
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id()
    );
    Ok(())
}
