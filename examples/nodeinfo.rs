// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example nodeinfo --release
use iota::Client;

/// In this example we get the nodeinfo
#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        // optional, because "testnet2" is default
        .with_network("testnet2")
        .with_node("http://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Nodeinfo: {:?}", info);
}
