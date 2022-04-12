// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example get_funds --release
use iota_client::{signing::mnemonic::MnemonicSigner, utils::request_funds_from_faucet, Client, Result};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let client = Client::builder()
        .with_node("http://localhost:14265") // Insert the node here
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = MnemonicSigner::new(&env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let addresses = client
        .get_addresses(&seed)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();
    println!("{}", addresses[0]);

    let faucet_response =
        request_funds_from_faucet("http://localhost:14265/api/plugins/faucet/v1/enqueue", &addresses[0]).await?;

    println!("{}", faucet_response);
    Ok(())
}
