// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 09_transaction --release

use iota_client::{Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will send a transaction

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")?
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();
    let seed_1 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);

    let message = iota
        .message()
        .with_seed(&seed_1)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            // We generate an address from our seed so that we send the funds to ourselves
            &iota.get_addresses(&seed_1).with_range(1..2).finish().await?[0],
            1_000_000,
        )?
        .finish()
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id().0
    );
    Ok(())
}
