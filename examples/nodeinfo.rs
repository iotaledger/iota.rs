// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example nodeinfo --release
use iota::Client;

/// In this example we get the nodeinfo
#[tokio::main]
async fn main() {
    let iota = Client::builder() // Create a client instance builder
        .with_node("http://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Nodeinfo: {:?}", info);
}
