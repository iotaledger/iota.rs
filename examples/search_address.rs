// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example search_address --release

use iota_client::{api::search_address, Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::{convert::TryInto, env};

/// In this example we will try to find the index and address type of an address

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

    let addresses = iota
        .get_addresses(&seed)
        .with_account_index(0)
        .with_range(9..10)
        .finish()
        .await
        .unwrap();

    println!("{:?}", addresses[0]);

    let res = search_address(
        &seed,
        &iota.get_bech32_hrp().await.unwrap(),
        0,
        0..10,
        &addresses[0].clone().try_into().unwrap(),
    )
    .await
    .unwrap();

    println!("Address index: {}\nIs internal address: {}", res.0, res.1);
}
