// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will send 1_000_000 tokens to atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r.
//! This address belongs to the first seed in .env.example.
//! Run: `cargo run --example custom_inputs --release`.

use iota_client::{
    block::input::UtxoInput, node_api::indexer::query_parameters::QueryParameter, request_funds_from_faucet,
    secret::SecretManager, Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production.
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();
    let faucet_url = std::env::var("FAUCET_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()?;

    // First address from the seed below is atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
    let secret_manager =
        SecretManager::try_from_hex_seed(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?;

    let addresses = client.get_addresses(&secret_manager).with_range(0..1).finish().await?;
    println!("{:?}", addresses[0]);

    println!("{}", request_funds_from_faucet(&faucet_url, &addresses[0]).await?);
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let output_ids_response = client
        .basic_output_ids(vec![QueryParameter::Address(addresses[0].clone())])
        .await?;
    println!("{output_ids_response:?}");

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(UtxoInput::from(output_ids_response.items[0]))?
        //.with_input_range(20..25)
        .with_output(
            "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r",
            1_000_000,
        )
        .await?
        .finish()
        .await?;

    println!("{block:#?}");

    println!(
        "Transaction sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );
    Ok(())
}
