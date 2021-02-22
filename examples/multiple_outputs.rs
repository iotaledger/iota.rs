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
///   output 0: 3_000_000 tokens atoi1qzj8s3kpacr6kmh05sxul4zp0xqulzn2vy9rznqj6rrc4nwd304pk6w523x
///   output 1: 2_800_000 tokens atoi1qzu7dnlfld2p0rhld20nr6axdnl0katmwu59fprwcnahglmnvgpwjsc20jg
///   output 2: 3_000_000 tokens atoi1qz0vue67w2e2wjk9jh07s7wfgxmsxgy9ssctn3nntyf9uqd6qs3zsp0k73u
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
    // First address from the seed below is atoi1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxtmtev5
    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed =
        Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap()).unwrap();

    let message = iota
        .message()
        .with_seed(&seed)
        .with_output(
            &"atoi1qzj8s3kpacr6kmh05sxul4zp0xqulzn2vy9rznqj6rrc4nwd304pk6w523x".into(),
            3_000_000,
        )
        .unwrap()
        .with_output(
            &"atoi1qzu7dnlfld2p0rhld20nr6axdnl0katmwu59fprwcnahglmnvgpwjsc20jg".into(),
            2_800_000,
        )
        .unwrap()
        .with_output(
            &"atoi1qz0vue67w2e2wjk9jh07s7wfgxmsxgy9ssctn3nntyf9uqd6qs3zsp0k73u".into(),
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
