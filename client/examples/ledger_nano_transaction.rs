// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example ledger_nano_transaction --features=ledger_nano --release
//! In this example we will create a transaction with a ledger nano hardware wallet.

use iota_client::{
    secret::{ledger_nano::LedgerSecretManager, SecretManager},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()?;

    let secret_manager = SecretManager::LedgerNano(LedgerSecretManager::new(true));

    // Generate addresses with custom account index and range
    let addresses = client
        .get_addresses(&secret_manager)
        .with_account_index(0)
        .with_range(0..2)
        .finish()
        .await?;

    println!("List of generated public addresses:\n{addresses:?}\n");

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            // We generate an address from our seed so that we send the funds to ourselves
            &addresses[1],
            1_000_000,
        )
        .await?
        .finish()
        .await?;

    println!("Transaction sent with block-id: {}", block.id());

    Ok(())
}
