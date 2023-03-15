// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_remainder_address --release
//! In this example we will send 9_000_000 tokens to a given receiver and 1_000_000 tokens to a custom remainder
//! address. The used addresses belong to the first seed in .env.example.

use iota_client::{
    node_api::indexer::query_parameters::QueryParameter, request_funds_from_faucet, secret::SecretManager, Client,
    Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()?;

    // First address from the seed below is atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
    let secret_manager =
        SecretManager::try_from_hex_seed(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?;

    let addresses = client.get_addresses(&secret_manager).with_range(0..3).finish().await?;

    let sender_address = &addresses[0];
    let receiver_address = &addresses[1];
    let remainder_address = &addresses[2];

    println!("sender address: {sender_address}");
    println!("receiver address: {receiver_address}");
    println!("remainder address: {remainder_address}");

    println!(
        "automatically funding sender address with faucet: {}",
        request_funds_from_faucet("http://localhost:8091/api/enqueue", sender_address).await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let output_ids_response = client
        .basic_output_ids(vec![QueryParameter::Address(sender_address.clone())])
        .await?;
    println!("{output_ids_response:?}");

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_output(
            // We generate an address from our seed so that we send the funds to ourselves
            receiver_address,
            9_000_000,
        )
        .await?
        // We send the remainder to an address where we don't have access to.
        .with_custom_remainder_address(remainder_address)?
        .finish()
        .await?;

    println!(
        "Transaction sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
