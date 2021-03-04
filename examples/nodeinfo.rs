// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example nodeinfo --release
use iota::Client;

/// In this example we get information about the node

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Node Info: {:?}", info);
}
