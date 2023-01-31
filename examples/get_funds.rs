// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example get_funds --release
use iota_client::{Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe") // Insert the node here
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

    let faucet_response = ureq::post("https://faucet.chrysalis-devnet.iota.cafe/api/plugins/faucet/enqueue")
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "address": addresses[0],
        }))
        .unwrap()
        .into_string()
        .unwrap();

    println!("{faucet_response}");
}
