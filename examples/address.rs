// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example address --release
use iota::{Client, Seed};
#[macro_use]
extern crate dotenv_codegen;

/// In this example we create addresses from a seed defined in .env
#[tokio::main]
async fn main() {
    let iota = Client::build() // Crate a client instance builder
        .with_node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .finish()
        .unwrap();

    let seed = Seed::from_ed25519_bytes(&hex::decode(dotenv!("seed")).unwrap()).unwrap();

    let addresses = iota
        .find_addresses(&seed)
        .with_account_index(0)
        .with_range(0..4)
        .finish()
        .unwrap();
    println!("List of generated public addresses: {:?}", addresses);
    let all_addresses = iota
        .find_addresses(&seed)
        .with_account_index(0)
        .with_range(0..4)
        .get_all()
        .unwrap();
    // bool for public addresses is false and for internal addresses true
    println!("List of generated public and internal addresses: {:?}", all_addresses);
}
