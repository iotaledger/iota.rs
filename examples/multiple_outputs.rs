// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example multiple_outputs --release
use iota::{Client, MessageId, Seed};
use std::time::Duration;
use tokio::time::sleep;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example, we send 8_800_000 tokens to the following 3 locations, respectively
///
/// Address Index (1..4)
///   output 0: 3_000_000 tokens atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf
///   output 1: 2_800_000 tokens atoi1qz4sfmp605vnj6fxt0sf0cwclffw5hpxjqkf6fthyd74r9nmmu337m3lwl2
///   output 2: 3_000_000 tokens atoi1qzumqjtucwglfja746vvmr7n54ep88kcu2qvaquqrnx9qs2z8f4t6d7muyq
///
///
/// These three addresses belong to second seed in .env.example

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        // .with_node("http://0.0.0.0:14265") // Insert the node here
        // .unwrap()
        .finish()
        .await
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    // First address from the seed below is atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());

    let message = iota
        .message()
        .with_seed(&seed)
        .with_output(
            &"atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf".into(),
            3_000_000,
        )
        .unwrap()
        .with_output(
            &"atoi1qz4sfmp605vnj6fxt0sf0cwclffw5hpxjqkf6fthyd74r9nmmu337m3lwl2".into(),
            2_800_000,
        )
        .unwrap()
        .with_output(
            &"atoi1qzumqjtucwglfja746vvmr7n54ep88kcu2qvaquqrnx9qs2z8f4t6d7muyq".into(),
            3_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;
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
