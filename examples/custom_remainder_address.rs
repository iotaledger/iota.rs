// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_inputs --release
use iota_client::{
    bee_message::input::UtxoInput, node_api::indexer_api::query_parameters::QueryParameter, request_funds_from_faucet,
    signing::mnemonic::MnemonicSigner, Client, Result,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

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
    let seed = MnemonicSigner::new_from_seed(&env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?;

    let addresses = client.get_addresses(&seed).with_range(0..1).finish().await?;
    println!("{:?}", addresses[0]);

    println!(
        "{}",
        request_funds_from_faucet("http://localhost:14265/api/plugins/faucet/v1/enqueue", &addresses[0]).await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    let output_ids = iota_client::node_api::indexer_api::routes::output_ids(
        &client,
        vec![QueryParameter::Address(addresses[0].clone())],
    )
        .await?;
    println!("{:?}", output_ids);

    let message = client
        .message()
        .with_signer(&seed)
        .with_output(
            // We generate an address from our seed so that we send the funds to ourselves
            &client.get_addresses(&seed).with_range(1..2).finish().await?[0],
            999_999,
        )?
        .with_custom_remainder_address("iota1qqjwn9jezjjgycfx9670s0v33krxr90h94zuxvdwr99l8h5ps7cxcr90dan")?
        .finish()
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id()
    );
    Ok(())
}
