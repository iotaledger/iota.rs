// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example search_address --release
use iota::{api::search_address, Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we try to find the index of an address from a seed.
#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed =
        Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap()).unwrap();

    let address = iota
        .find_addresses(&seed)
        .with_account_index(0)
        .with_range(9..10)
        .finish()
        .await
        .unwrap();
    println!("{:?}", address);
    let res = search_address(&seed, iota.get_bech32_hrp().await.unwrap(), 0, 0..10, &address[0])
        .await
        .unwrap();
    println!(
        "Found address with address_index: {}, internal address: {}",
        res.0, res.1
    );
}
