// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example indexer --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    node_api::indexer::query_parameters::QueryParameter,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    utils::request_funds_from_faucet,
    Client, Result,
};

/// In this example we will get output ids from the indexer API

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic must contain non-zero
    // balance
    dotenv().ok();

    let node_url = env::var("NODE_URL").unwrap();
    let faucet_url = env::var("FAUCET_URL").unwrap();

    let client = Client::builder()
        .with_node(&node_url)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap(),
    )?);

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];

    println!(
        "{}",
        request_funds_from_faucet(&faucet_url, &address.to_bech32("atoi")).await?
    );

    let output_ids = client
        .basic_output_ids(vec![QueryParameter::Address(address.to_bech32("atoi"))])
        .await?;
    println!("output ids {:?}", output_ids);

    let outputs = client.get_outputs(output_ids).await?;

    println!("outputs {:?}", outputs);
    Ok(())
}
