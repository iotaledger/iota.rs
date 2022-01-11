// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example get_funds --release
use iota_client::{
    signing::{mnemonic::MnemonicSigner, SignerHandle},
    Client, Result,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265") // Insert the node here
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = SignerHandle::new(Box::new(MnemonicSigner::new_from_seed(
        &env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap(),
    )?));
    let addresses = iota
        .get_addresses(&seed)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();
    println!("{}", addresses[0]);

    let faucet_response = ureq::post("http://localhost:14265/api/plugins/faucet/enqueue")
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "address": addresses[0],
        }))
        .unwrap()
        .into_string()
        .unwrap();

    println!("{}", faucet_response);
    Ok(())
}
