// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example txspam --release

use iota_client::{
    bee_message::prelude::{Essence, Payload, UtxoInput},
    Client, Result, Seed,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will spam transactions
/// Send 10 Mi from the faucet to the first address before you run it

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")? // Insert your node URL here
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);

    // Split funds to own addresses
    let addresses = iota
        .get_addresses(&seed)
        .with_account_index(0)
        .with_range(0..10)
        .finish()
        .await?;

    let mut message_builder = iota.message().with_seed(&seed);
    for address in &addresses {
        message_builder = message_builder.with_output(address, 1_000_000)?;
    }
    let message = message_builder.finish().await?;

    println!(
        "First transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id().0
    );

    let _ = iota.retry_until_included(&message.id().0, None, None).await?;

    // At this point we have 10 Mi on 10 addresses and we will just send it to their addresses again
    // Use own outputs directly so we don't double spend them
    let mut initial_outputs = Vec::new();
    if let Some(Payload::Transaction(tx)) = message.payload() {
        let Essence::Regular(essence) = tx.essence();
        for (index, _output) in essence.outputs().iter().enumerate() {
            initial_outputs.push(UtxoInput::new(tx.id(), index as u16)?);
        }
    }

    for (index, address) in addresses.into_iter().enumerate() {
        let message = iota
            .message()
            .with_seed(&seed)
            .with_input(initial_outputs[index].clone())
            .with_output(&address, 1_000_000)?
            .finish()
            .await?;
        println!(
            "Transaction sent: https://explorer.iota.org/devnet/message/{}",
            message.id().0
        );
    }
    Ok(())
}
