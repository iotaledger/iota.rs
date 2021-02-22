// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example transaction --release
use iota::{Client, MessageId, Seed};
use std::time::Duration;
use tokio::time::sleep;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example, we send 9_000_000 tokens to the following 3 locations, respectively
/// First send 10 Mi from the faucet to atoi1qzj86lzml2ktagye4mj0th6zymgka8lt96qre9yye0v8sawzmdu0ut90vm7
/// That's the first address of the first seed in the .env.example file
///
/// Address Index 0. Note that we can use the `address` example codes to know the addresses belong to the seed.
/// Outputs we send to the first addresses from the second seed:
///   output 0: 3_000_000 tokens atoi1qzj8s3kpacr6kmh05sxul4zp0xqulzn2vy9rznqj6rrc4nwd304pk6w523x
///   output 1: 3_000_000 tokens atoi1qzu7dnlfld2p0rhld20nr6axdnl0katmwu59fprwcnahglmnvgpwjsc20jg
///   output 2: 3_000_000 tokens atoi1qz0vue67w2e2wjk9jh07s7wfgxmsxgy9ssctn3nntyf9uqd6qs3zsp0k73u
///
///
/// Then we send 6_000_000 tokens from the second seed to the first one
/// to addresses "atoi1qrdypwg7lrghd9urpuhtsx5mt2r27wxma5nevcr39pszkqejt00dgku89p7" and
/// "atoi1qp9zlm8j628qp4x9853hcvuuwy6m85kllzv5fcveex67ym8ut7peu9tnm7r", and check the ledger
/// inclusion state, which should be "included".

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://localhost:14265") // Insert the node here
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();

    // Insert your seed in the .env. Since the output amount cannot be zero. The seed must contain non-zero balance.
    // First address from the seed in the .env is iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4
    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed =
        Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap()).unwrap();

    let message = iota
        .message()
        .with_seed(&seed)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            &"atoi1qzj8s3kpacr6kmh05sxul4zp0xqulzn2vy9rznqj6rrc4nwd304pk6w523x".into(),
            3_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "First transaction sent: http://127.0.0.1:14265/api/v1/messages/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;

    let message = iota
        .message()
        .with_seed(&seed)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            &"atoi1qzu7dnlfld2p0rhld20nr6axdnl0katmwu59fprwcnahglmnvgpwjsc20jg".into(),
            3_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Second transaction sent: http://127.0.0.1:14265/api/v1/messages/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;

    let message = iota
        .message()
        .with_seed(&seed)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            &"atoi1qz0vue67w2e2wjk9jh07s7wfgxmsxgy9ssctn3nntyf9uqd6qs3zsp0k73u".into(),
            3_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();
    println!(
        "Third transaction sent: http://127.0.0.1:14265/api/v1/messages/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;

    let seed =
        Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap()).unwrap()).unwrap();

    let message = iota
        .message()
        .with_seed(&seed)
        // Insert the output address and amount to spent. The amount cannot be zero.
        // Note that we can transfer to multiple outputs by using the `SendTransactionBuilder`
        .with_output(
            &"atoi1qrdypwg7lrghd9urpuhtsx5mt2r27wxma5nevcr39pszkqejt00dgku89p7".into(),
            3_000_000,
        )
        .unwrap()
        .with_output(
            &"atoi1qp9zlm8j628qp4x9853hcvuuwy6m85kllzv5fcveex67ym8ut7peu9tnm7r".into(),
            3_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Last transaction sent: http://127.0.0.1:14265/api/v1/messages/{}",
        message.id().0
    );
    reattach_promote_until_confirmed(message.id().0, &iota).await;
    let message_metadata = iota.get_message().metadata(&message.id().0).await;
    println!(
        "The ledgerInclusionState: {:?}",
        message_metadata.unwrap().ledger_inclusion_state
    );
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
