// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example address --release
use iota::{api::GetAddressesBuilder, Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we create addresses from a seed defined in .env
#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());

    // Generate addresses with default account index and range
    let addresses = iota.get_addresses(&seed).finish().await.unwrap();
    println!("List of generated public addresses:\n{:?}\n", addresses);

    // Generate addresses with custom account index and range
    let addresses = iota
        .get_addresses(&seed)
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await
        .unwrap();
    println!("List of generated public addresses:\n{:?}\n", addresses);

    // Generate public (false) & internal (true) addresses
    let addresses = iota.get_addresses(&seed).with_range(0..4).get_all().await.unwrap();
    println!("List of generated public and internal addresses:\n{:?}\n", addresses);

    // Generate public addresses offline with the bech32_hrp defined
    let addresses = GetAddressesBuilder::new(&seed)
        .with_bech32_hrp("atoi".into())
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await
        .unwrap();
    println!("List of offline generated public addresses:\n{:?}\n", addresses);
}
