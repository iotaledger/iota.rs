// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_inputs --release
use iota_client::{
    bee_message::{input::UtxoInput, output::OutputId},
    Client, Result, Seed,
};
extern crate dotenv;
use dotenv::dotenv;
use std::{env, str::FromStr};

/// In this example we will send 1_000_000 tokens to atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
/// This address belongs to the first seed in .env.example

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    // First address from the seed below is atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);

    let addresses = iota.get_addresses(&seed).with_range(0..1).finish().await?;
    println!("{:?}", addresses[0]);

    let outputs = iota
        .get_address()
        .outputs_response(&addresses[0], Default::default())
        .await?;
    println!("{:?}", outputs);

    let message = iota
        .message()
        .with_seed(&seed)
        .with_input(UtxoInput::from(OutputId::from_str(&outputs.output_ids[0])?))
        //.with_input_range(20..25)
        .with_output(
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r",
            1_000_000,
        )?
        .finish()
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id()
    );
    Ok(())
}
