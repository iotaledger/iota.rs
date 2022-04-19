// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_inputs --release
use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_message::input::UtxoInput, node_api::indexer::query_parameters::QueryParameter, request_funds_from_faucet,
    secret::mnemonic::MnemonicSecretManager, Client, Result,
};

/// In this example we will send 1_000_000 tokens to atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
/// This address belongs to the first seed in .env.example

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let client = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    // First address from the seed below is atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r
    let secmngr = MnemonicSecretManager::try_from_hex_seed(&env::var("NON_SECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?;

    let addresses = client.get_addresses(&secmngr).with_range(0..1).finish().await?;
    println!("{:?}", addresses[0]);

    println!(
        "{}",
        request_funds_from_faucet("http://localhost:14265/api/plugins/faucet/v1/enqueue", &addresses[0]).await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let output_ids = iota_client::node_api::indexer::routes::output_ids(
        &client,
        vec![QueryParameter::Address(addresses[0].clone())],
    )
    .await?;
    println!("{:?}", output_ids);

    let message = client
        .message()
        .with_secret_manager(&secmngr)
        .with_input(UtxoInput::from(output_ids[0]))?
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
