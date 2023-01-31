// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 03_generate_addresses --release

use iota_client::{api::GetAddressesBuilder, Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will create addresses from a seed defined in .env

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());

    // Generate addresses with default account index and range
    let addresses = iota.get_addresses(&seed).finish().await.unwrap();
    println!("List of generated public addresses:\n{addresses:?}\n");

    // Generate addresses with custom account index and range
    let addresses = iota
        .get_addresses(&seed)
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await
        .unwrap();

    println!("List of generated public addresses:\n{addresses:?}\n");

    // Generate public (false) & internal (true) addresses
    let addresses = iota.get_addresses(&seed).with_range(0..4).get_all().await.unwrap();
    println!("List of generated public and internal addresses:\n{addresses:?}\n");

    // Generate public addresses offline with the bech32_hrp defined
    let addresses = GetAddressesBuilder::new(&seed)
        .with_bech32_hrp("atoi".into())
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .await
        .unwrap();

    println!("List of offline generated public addresses:\n{addresses:?}\n");
}
