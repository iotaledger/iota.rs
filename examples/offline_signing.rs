// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example offline_signing --release
use iota_client::{Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will send a dust allowance output and dust

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota_offline = Client::builder().with_offline_mode().finish().await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);

    // Generate address offline
    let addresses = iota_offline
        .get_addresses(&seed)
        .with_range(1..2)
        .with_bech32_hrp("atoi".into())
        .finish()
        .await?;
    println!("{:?}", addresses[0]);

    // Get inputs and create transaction essence online
    let iota_online = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com")? // Insert your node URL here
        .finish()
        .await?;

    let outputs = iota_online
        .get_address()
        .outputs(&addresses[0], Default::default())
        .await?;
    println!("{:?}", outputs);

    let (essence, address_index_recorders) = iota_online
        .message()
        .with_input(outputs[0].clone())
        .with_output(
            &"atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r",
            1000000,
        )?
        .prepare_transaction()
        .await?;

    // Sign prepared transaction offline
    let signed_transaction = iota_offline
        .message()
        .sign_transaction(essence, address_index_recorders, Some(&seed), None)
        .await?;

    // Send offline signed transaction online
    let message = iota_online.message().finish_message(Some(signed_transaction)).await?;

    println!(
        "Transaction sent: https://explorer.iota.org/testnet/message/{}",
        message.id().0
    );
    Ok(())
}
