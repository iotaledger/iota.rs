// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example txspam --release
use iota::{Client, Essence, MessageId, Payload, Seed, UTXOInput};
use tokio::time::sleep;
extern crate dotenv;
use dotenv::dotenv;
use std::{env, time::Duration};

/// In this example, we spam transactions
/// Send 10 Mi from the faucet to the first address before you run this

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Create a client instance builder
        .with_node("http://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed =
        Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap()).unwrap();

    // split funds to own addresses
    let addresses = iota
        .find_addresses(&seed)
        .with_account_index(0)
        .with_range(0..10)
        .finish()
        .await
        .unwrap();

    let mut message_builder = iota.message().with_seed(&seed);
    for address in &addresses {
        message_builder = message_builder.with_output(address, 1_000_000).unwrap();
    }
    let message = message_builder.finish().await.unwrap();

    println!(
        "First transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;
    // At this point we have 10 Mi on 10 addresses and will just send it to their addresses again

    // Use own outputs directly so we don't double spend them
    let mut initial_outputs = Vec::new();
    if let Some(Payload::Transaction(tx)) = message.payload() {
        match tx.essence() {
            Essence::Regular(essence) => {
                for (index, _output) in essence.outputs().iter().enumerate() {
                    initial_outputs.push(UTXOInput::new(tx.id(), index as u16).unwrap());
                }
            }
            _ => {
                panic!("Unexisting essence type");
            }
        }
    }

    for (index, address) in addresses.iter().enumerate() {
        let message = iota
            .message()
            .with_seed(&seed)
            .with_input(initial_outputs[index].clone())
            .with_output(address, 1_000_000)
            .unwrap()
            .finish()
            .await
            .unwrap();
        println!(
            "Tx sent: https://explorer.iota.org/chrysalis/message/{}",
            message.id().0
        );
    }
}

async fn reattach_promote_until_confirmed(message_id: MessageId, iota: &Client) {
    while let Ok(metadata) = iota.get_message().metadata(&message_id).await {
        if let Some(state) = metadata.ledger_inclusion_state {
            println!("Ledger inclusion state: {:?}", state);
            break;
        } else if let Ok(msg_id) = iota.reattach(&message_id).await {
            println!("Reattached or promoted {}", msg_id.0);
        }
        sleep(Duration::from_secs(5)).await;
    }
}
