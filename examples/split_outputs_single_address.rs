// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example split_outputs_single_address --release

use iota::{Client, Essence, MessageId, Output, Payload, Seed, UTXOInput};
use tokio::time::sleep;
extern crate dotenv;
use dotenv::dotenv;
use std::{env, time::Duration};

/// In this example we will create 100 outputs on a single address
/// You need to have >=100 Mi on the first address before you run it

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap()).unwrap());

    // Split funds to own addresses
    let addresses = iota
        .get_addresses(&seed)
        // We start from index 1 so we can send remaining balance to the address with index 0
        .with_range(1..101)
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

    // At this point we have 100 Mi on 100 addresses and we will just send it to the final address
    // We use the outputs directly so we don't double spend them
    let mut initial_outputs = Vec::new();
    if let Some(Payload::Transaction(tx)) = message.payload() {
        match tx.essence() {
            Essence::Regular(essence) => {
                for (index, output) in essence.outputs().iter().enumerate() {
                    // Only include 1 Mi outputs, otherwise it fails for the remainder address
                    if let Output::SignatureLockedSingle(output) = output {
                        if output.amount() == 1_000_000 {
                            initial_outputs.push(UTXOInput::new(tx.id(), index as u16).unwrap());
                        }
                    }
                }
            }
            _ => {
                panic!("Non-existing essence type");
            }
        }
    }
    let mut sent_messages = Vec::new();
    for (index, output) in initial_outputs.into_iter().enumerate() {
        let message_id = iota
            .message()
            .with_seed(&seed)
            .with_input(output)
            .with_input_range(1..101)
            .with_output(
                &"atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".into(),
                1_000_000,
            )
            .unwrap()
            .finish()
            .await
            .unwrap()
            .id()
            .0;
        println!(
            "Transaction {} sent: https://explorer.iota.org/chrysalis/message/{}",
            index, message_id
        );
        sent_messages.push(message_id);
    }
    for message_id in sent_messages {
        reattach_promote_until_confirmed(message_id, &iota).await
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
