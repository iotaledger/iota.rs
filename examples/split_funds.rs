// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example split_funds --release

use iota_client::{signing::mnemonic::MnemonicSigner, Client, Result};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will send a transaction

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();
    let signer = MnemonicSigner::new(&env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap())?;

    let mut message_builder = iota.message().with_signer(&signer);
    // Insert the output address and amount to spent. The amount cannot be zero.
    for _ in 0..100 {
        message_builder = message_builder.with_output(
            // We generate an address from our seed so that we send the funds to ourselves
            &iota.get_addresses(&signer).with_range(0..1).finish().await?[0],
            1_000_000,
        )?
    }
    let message = message_builder.finish().await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id()
    );
    Ok(())
}
