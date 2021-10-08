// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example transaction --release
use iota_client::{Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will send 9_000_000 tokens to the following 3 locations, respectively
/// First send 10 Mi from the faucet to atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
/// That's the first address of the first seed in the .env.example file
///
/// Address Index 0. Note that we can use the `address` example codes to know the addresses belong to the seed.
/// Outputs we send to the first addresses from the second seed:
///   output 0: 3_000_000 tokens atoi1qzj8s3kpacr6kmh05sxul4zp0xqulzn2vy9rznqj6rrc4nwd304pk6w523x
///   output 1: 3_000_000 tokens atoi1qzu7dnlfld2p0rhld20nr6axdnl0katmwu59fprwcnahglmnvgpwjsc20jg
///   output 2: 3_000_000 tokens atoi1qz0vue67w2e2wjk9jh07s7wfgxmsxgy9ssctn3nntyf9uqd6qs3zsp0k73u
///
///
/// Then we will send 6_000_000 tokens from the second seed to the first one
/// to addresses "atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf" (index 1) and
/// "atoi1qz4sfmp605vnj6fxt0sf0cwclffw5hpxjqkf6fthyd74r9nmmu337m3lwl2" (index 2), and check the ledger
/// inclusion state, which should be "Some(Included)".

const EXPLORER_URL: &str = "https://explorer.iota.org/devnet/message/";

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")? // Insert your node URL here
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();

    let seed_1 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);
    let seed_2 = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap())?);

    let message = iota
        .message()
        .with_seed(&seed_1)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_output(
            &iota.get_addresses(&seed_2).with_range(0..1).finish().await?[0],
            3_000_000,
        )?
        .finish()
        .await?;

    println!("First transaction sent: {}{}", EXPLORER_URL, message.id().0);
    let _ = iota.retry_until_included(&message.id().0, None, None).await?;

    let message = iota
        .message()
        .with_seed(&seed_1)
        .with_output(
            &iota.get_addresses(&seed_2).with_range(1..2).finish().await?[0],
            3_000_000,
        )?
        .finish()
        .await?;

    println!("Second transaction sent: {}{}", EXPLORER_URL, message.id().0);
    let _ = iota.retry_until_included(&message.id().0, None, None).await?;

    let message = iota
        .message()
        .with_seed(&seed_1)
        .with_output(
            &iota.get_addresses(&seed_2).with_range(2..3).finish().await?[0],
            3_000_000,
        )?
        .finish()
        .await?;

    println!("Third transaction sent: {}{}", EXPLORER_URL, message.id().0);
    let _ = iota.retry_until_included(&message.id().0, None, None).await?;

    let message = iota
        .message()
        .with_seed(&seed_2)
        // Note that we can transfer to multiple outputs by using the `SendTransactionBuilder`
        .with_output(
            &iota.get_addresses(&seed_1).with_range(1..2).finish().await?[0],
            3_000_000,
        )?
        .with_output(
            &iota.get_addresses(&seed_1).with_range(2..3).finish().await?[0],
            3_000_000,
        )?
        .finish()
        .await?;

    println!("Last transaction sent: {}{}", EXPLORER_URL, message.id().0);
    let _ = iota.retry_until_included(&message.id().0, None, None).await?;

    let message_metadata = iota.get_message().metadata(&message.id().0).await;
    println!("Ledger Inclusion State: {:?}", message_metadata?.ledger_inclusion_state);

    Ok(())
}
