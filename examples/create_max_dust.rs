// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example create_max_dust --release

use iota_client::{
    bee_message::prelude::{Essence, Output, Payload, UtxoInput},
    Client, Result, Seed,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will create 100 dust outputs on the first address of the second seed
/// For this example you need to have >100 Mi on the first address before you run it
/// because we send transactions in parallel

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")?
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);
    let seed_2 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap())?);

    let new_address = iota.get_addresses(&seed_2).with_range(0..1).finish().await?;

    let dust_allowance_message = iota
        .message()
        .with_seed(&seed)
        .with_dust_allowance_output(&new_address[0], 10_000_000)?
        .finish()
        .await?;
    let _ = iota
        .retry_until_included(&dust_allowance_message.id().0, None, None)
        .await?;

    // Split funds to own addresses
    let addresses = iota
        .get_addresses(&seed)
        // We start from index 1 so we can send remaining balance to the address with index 0
        .with_range(1..101)
        .finish()
        .await?;

    let mut message_builder = iota.message().with_seed(&seed);
    for address in addresses {
        message_builder = message_builder.with_output(&address, 1_000_001)?;
    }
    let message = message_builder.finish().await?;

    println!(
        "First transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id().0
    );

    let _ = iota.retry_until_included(&message.id().0, None, None).await?;

    // At this point we have 100 Mi on 100 addresses and we will just send it to the final address
    // We use the outputs directly so we don't double spend them
    let mut initial_outputs = Vec::new();
    if let Some(Payload::Transaction(tx)) = message.payload() {
        let Essence::Regular(essence) = tx.essence();
        for (index, output) in essence.outputs().iter().enumerate() {
            // Only include 1 Mi outputs, otherwise it fails for the remainder address
            if let Output::SignatureLockedSingle(output) = output {
                if output.amount() == 1_000_001 {
                    initial_outputs.push(UtxoInput::new(tx.id(), index as u16)?);
                }
            }
        }
    }

    let first_address_old_seed = iota.get_addresses(&seed).with_range(0..1).finish().await?;
    let mut sent_messages = Vec::new();
    for (index, output) in initial_outputs.into_iter().enumerate() {
        let message_id = iota
            .message()
            .with_seed(&seed)
            .with_input(output)
            .with_input_range(1..101)
            .with_output(&new_address[0], 1)?
            // send remaining iotas back
            .with_output(&first_address_old_seed[0], 1_000_000)?
            .finish()
            .await?
            .id()
            .0;
        println!("Transaction {index} sent: https://explorer.iota.org/devnet/message/{message_id}");
        sent_messages.push(message_id);
    }
    // only check last message, if this gets confirmed all other messages should also be confirmed
    let _ = iota
        .retry_until_included(&sent_messages.pop().unwrap(), None, None)
        .await?;
    // Send all funds back to first address
    let total_balance = iota.get_balance(&seed).finish().await?;

    println!("Total balance: {total_balance}");

    let message = iota
        .message()
        .with_seed(&seed)
        .with_output(&first_address_old_seed[0], total_balance)?
        .finish()
        .await?;

    println!(
        "Final tx sent: https://explorer.iota.org/devnet/message/{}",
        message.id().0
    );

    let _ = iota.retry_until_included(&message.id().0, None, None).await.unwrap();
    Ok(())
}
