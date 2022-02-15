// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example split_funds --release

use iota_client::{request_funds_from_faucet, signing::mnemonic::MnemonicSigner, Client, Result};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will send 100 basic outputs to our first address

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

    let address = iota.get_addresses(&signer).with_range(0..1).get_raw().await?[0];
    println!(
        "{}",
        request_funds_from_faucet(
            "http://localhost:14265/api/plugins/faucet/v1/enqueue",
            &address.to_bech32("atoi"),
        )
        .await?
    );

    // wait so the faucet can send the funds
    // tokio::time::sleep(std::time::Duration::from_secs(20)).await;

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
        "Transaction sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );
    Ok(())
}
