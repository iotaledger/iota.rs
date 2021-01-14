// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example multiple_outputs --release
use iota::{Client, MessageId, Seed};
use std::time::Duration;
use tokio::time::sleep;
#[macro_use]
extern crate dotenv_codegen;

/// In this example, we send 900 tokens to the following 3 locations, respectively
///
/// Address Index 0
///   output 0: 300 tokens iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6
///   output 1: 300 tokens iot1qyg7l34etk4sdfrdt46vwt7a964avk9sfrxh8ecq2sgpezaktd55cyc76lc
///   output 2: 300 tokens iot1q9r5hvlppf44gvcxnuue4dwjtjcredrw6yesphqeq7fqm2fyjy6kul4tv5r
///
///
/// These two addresses belong to seed "256a818b2aac458941f7274985a410e57fb750f3a3a67369ece5bd9ae7eef5b0"

#[tokio::main]
async fn main() {
    let iota = Client::build() // Crate a client instance builder
        .with_node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .finish()
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    // First address from the seed below is iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4
    let seed = Seed::from_ed25519_bytes(&hex::decode(dotenv!("seed")).unwrap()).unwrap();

    let message_id = iota
        .send()
        .with_seed(&seed)
        .with_output(
            &"iot1q86rlrygq5wcgdwt7fpajaxxppc49tg0jk0xadnp66fsfjtwt8vgc48sse6".into(),
            300,
        )
        .unwrap()
        .with_output(
            &"iot1qyg7l34etk4sdfrdt46vwt7a964avk9sfrxh8ecq2sgpezaktd55cyc76lc".into(),
            280,
        )
        .unwrap()
        .with_output(
            &"iot1q9r5hvlppf44gvcxnuue4dwjtjcredrw6yesphqeq7fqm2fyjy6kul4tv5r".into(),
            300,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message_id
    );
    reattach_promote_until_confirmed(message_id, &iota).await;
}

async fn reattach_promote_until_confirmed(message_id: MessageId, iota: &Client) {
    while let Ok(metadata) = iota.get_message().metadata(&message_id).await {
        if let Some(state) = metadata.ledger_inclusion_state {
            println!("Leder inclusion state: {}", state);
            break;
        } else if let Ok(msg_id) = iota.reattach(&message_id).await {
            println!("Reattached or promoted {}", msg_id.0);
        }
        sleep(Duration::from_secs(5)).await;
    }
}
