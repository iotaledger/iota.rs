// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example multiple_outputs --release
use iota_client::{Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will send 8_800_000 tokens to the following 3 locations, respectively
///
/// Address Index (1..4)
///   output 0: 3_000_000 tokens atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf
///   output 1: 2_800_000 tokens atoi1qz4sfmp605vnj6fxt0sf0cwclffw5hpxjqkf6fthyd74r9nmmu337m3lwl2
///   output 2: 3_000_000 tokens atoi1qzumqjtucwglfja746vvmr7n54ep88kcu2qvaquqrnx9qs2z8f4t6d7muyq
///
///
/// Those three addresses belong to second seed in .env.example

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")? // Insert your node URL here
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();

    // Seed must contain non-zero balance.
    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);

    let message = iota
        .message()
        .with_seed(&seed)
        .with_output(
            "atoi1qpnrumvaex24dy0duulp4q07lpa00w20ze6jfd0xly422kdcjxzakzsz5kf",
            3_000_000,
        )?
        .with_output(
            "atoi1qz4sfmp605vnj6fxt0sf0cwclffw5hpxjqkf6fthyd74r9nmmu337m3lwl2",
            2_800_000,
        )?
        .with_output(
            "atoi1qzumqjtucwglfja746vvmr7n54ep88kcu2qvaquqrnx9qs2z8f4t6d7muyq",
            3_000_000,
        )?
        .finish()
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id().0
    );

    let _ = iota.retry_until_included(&message.id().0, None, None).await?;
    Ok(())
}
