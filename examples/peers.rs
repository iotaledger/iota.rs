// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example peers --release
use iota::Client;

/// In this example we get the nodeinfo
#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let peers = iota.get_peers().await.unwrap();
    println!("Peers: {:?}", peers);
}
