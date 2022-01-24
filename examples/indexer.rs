// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example indexer --release

use iota_client::{
    node_api::indexer_api::query_parameters::{QueryParameter, QueryParameters},
    signing::mnemonic::MnemonicSigner,
    utils::{init_logger, request_funds_from_faucet, LevelFilter},
    Client, Result,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get output ids from the indexer API

#[tokio::main]
async fn main() -> Result<()> {
    init_logger("iota.rs.log", LevelFilter::Debug)?;
    let iota = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();
    let signer = MnemonicSigner::new(&env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap())?;

    let address = iota.get_addresses(&signer).with_range(0..1).get_all_raw().await?.public[0];

    println!(
        "{}",
        request_funds_from_faucet(
            "http://localhost:14265/api/plugins/faucet/enqueue",
            &address.to_bech32("atoi"),
        )
        .await?
    );

    let output_ids = iota_client::node_api::indexer_api::routes::output_ids(
        &iota,
        QueryParameters::new(vec![QueryParameter::Address(address.to_bech32("atoi"))]),
    )
    .await?;
    println!("output ids {:?}", output_ids);

    let outputs = iota_client::node_api::core_api::get_outputs(iota, output_ids).await?;

    println!("outputs {:?}", outputs);
    Ok(())
}
