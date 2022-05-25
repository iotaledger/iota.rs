// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example ledger_nano --features=ledger_nano --release

use iota_client::{
    secret::{ledger_nano::LedgerSecretManager, SecretManager},
    Client, Result,
};

/// In this example we will create addresses with a ledger nano hardware wallet

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let client = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_node_sync_disabled()
        .finish()
        .await?;

    let secret_manager = SecretManager::LedgerNano(LedgerSecretManager::new(true));

    // Generate addresses with custom account index and range
    let addresses = client
        .get_addresses(&secret_manager)
        .with_account_index(0)
        .with_range(0..2)
        .finish()
        .await?;

    println!("List of generated public addresses:\n{:?}\n", addresses);

    let block = client
        .block()
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
        "Transaction sent: https://explorer.alphanet.iotaledger.net/alphanet/block/{}",
        block.id()
    );


    Ok(())
}
