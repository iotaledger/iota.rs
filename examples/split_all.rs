// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example split_all --release

use iota_client::{Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get the balance of the first account of the seed and send everything splitted to the second
/// seed

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

    let seed_1 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);
    let seed_2 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap())?);

    let total_balance = iota.get_balance(&seed_1).finish().await?;
    let mut available = total_balance;

    println!("Total balance: {total_balance}i");

    let addresses_from_seed_2 = iota
        .get_addresses(&seed_2)
        .with_range(0..available as usize / 1_000_000)
        .finish()
        .await?;

    let mut message_builder = iota.message().with_seed(&seed_1);

    for i in 0..total_balance / 1_000_000 {
        let mut amount = 1_000_000;
        // Don't add more than we have or is allowed; One less here for remaining iotas
        if available == 0 || i > 125 {
            break;
        }
        available -= amount;
        // Add last amount so we don't create dust
        if available < amount {
            amount += available;
            available = 0;
        }
        message_builder = message_builder.with_output(&addresses_from_seed_2[i as usize], amount)?;
    }

    let message = message_builder.finish().await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id().0
    );

    let _ = iota.retry_until_included(&message.id().0, None, None).await.unwrap();
    Ok(())
}
