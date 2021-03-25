// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example get_funds --release
use iota::{Client, Seed};
use serde::Deserialize;
use std::time::Duration;
use tokio::time::sleep;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Deserialize)]
struct FaucetMessageResponse {
    id: String,
}

#[derive(Debug, Deserialize)]
struct FaucetResponse {
    data: FaucetMessageResponse,
}

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());
    let addresses = iota
        .get_addresses(&seed)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();
    println!("{}", addresses[0]);
    for i in 0..1 {
        let response: FaucetResponse = ureq::get(&format!(
            "https://faucet.testnet.chrysalis2.com/api?address={}",
            addresses[0]
        ))
        .call()
        .unwrap()
        .into_json()
        .unwrap();
        println!("{}: {:?}", i, response);
        // Faucet spam protection time
        sleep(Duration::from_secs(60)).await;
    }
}
