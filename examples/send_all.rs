// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example send_all --release

use iota_client::{Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get the balance of the first account of the seed and send everything
/// Run the consolidation example first if there are more than 127 outputs

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")? // Insert your node URL here
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);
    let seed_2 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap())?);
    let total_balance = iota.get_balance(&seed).with_initial_address_index(0).finish().await?;

    println!("Total balance: {total_balance}");

    let message = iota
        .message()
        .with_seed(&seed)
        .with_output(
            &iota.get_addresses(&seed_2).with_range(0..1).finish().await?[0],
            total_balance,
        )?
        .finish()
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id().0
    );

    let _ = iota.retry_until_included(&message.id().0, None, None).await.unwrap();
    Ok(())
}
