// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example balance --release
use iota::Client;

/// In this example we will get the balance of a known address
#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://localhost:14265") // Insert the node here
        .unwrap()
        .with_network("testnet3")
        .with_node_sync_disabled()
        .finish()
        .unwrap();

    let address = "atoi1q95jpvtk7cf7c7l9ne50c684jl4n8ya0srm5clpak7qes9ratu0ey2k2yn4";

    let balance = iota.get_address().balance(&address.into()).await.unwrap();
    println!("The balance of {:?} is {:?}", address, balance);

    let outputs = iota.get_address().outputs(&address.into()).await.unwrap();
    println!("The outputs of {:?} are {:?}", address, outputs);

    let output = iota.get_output(&outputs[0]).await.unwrap();
    println!("Output {:?}", output);
}
