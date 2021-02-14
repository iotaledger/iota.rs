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

    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed =
        Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap()).unwrap();

    let addresses = iota
        .find_addresses(&seed)
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await
        .unwrap();
    println!("List of generated public addresses: {:?}", addresses);
    let all_addresses = iota
        .find_addresses(&seed)
        .with_account_index(0)
        .with_range(0..4)
        .get_all()
        .await
        .unwrap();
    // bool for public addresses is false and for internal addresses true
    println!("List of generated public and internal addresses: {:?}", all_addresses);

    // Or generate addresses offline with the bech32_hrp defined
    let addresses = GetAddressesBuilder::new(&seed)
        .with_bech32_hrp("atoi".into())
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();
    println!("{:?}", addresses);
}
