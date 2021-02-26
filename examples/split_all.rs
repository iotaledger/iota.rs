// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example split_all --release
use iota::{client::Result, Client, MessageId, Seed};
use std::time::Duration;
use tokio::time::sleep;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example, we get the balance of the first account of the seed and send everything splitted to the second seed

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://api.lb-0.testnet.chrysalis2.com")?
        .finish()
        .await?;

    // Insert your seed in the .env. Since the output amount cannot be zero. The seed must contain non-zero balance.
    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed_1 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);
    let seed_2 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap())?);

    let total_balance = iota.get_balance(&seed_1).finish().await?;
    let mut available = total_balance;
    println!("total_balance {}", total_balance);
    let addresses_from_seed_2 = iota
        .get_addresses(&seed_2)
        .with_range(0..available as usize / 1_000_000)
        .finish()
        .await?;
    let mut message_builder = iota.message().with_seed(&seed_1);
    for i in 0..total_balance / 1_000_000 {
        let mut amount = 1_000_000;
        // Don't add more than we have or is allowed; 1 less here for remaining iotas
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
        "Transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;
    Ok(())
}

async fn reattach_promote_until_confirmed(message_id: MessageId, iota: &Client) {
    while let Ok(metadata) = iota.get_message().metadata(&message_id).await {
        if let Some(state) = metadata.ledger_inclusion_state {
            println!("Leder inclusion state: {:?}", state);
            break;
        } else if let Ok(msg_id) = iota.reattach(&message_id).await {
            println!("Reattached or promoted {}", msg_id.0);
        }
        sleep(Duration::from_secs(5)).await;
    }
}
