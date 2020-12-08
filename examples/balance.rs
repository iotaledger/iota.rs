// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota::{Client, Ed25519Address};

use std::convert::TryInto;

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .build()
        .unwrap();

    let address = Ed25519Address::new(
        hex::decode("6920b176f613ec7be59e68fc68f597eb3393af80f74c7c3db78198147d5f1f92") // Insert the address to search for
            .unwrap()
            .try_into()
            .unwrap(),
    )
    .into();
    let balance = iota.get_address().balance(&address).await.unwrap();
    println!("The balance of {:?} is {:?}", address, balance);

    let outputs = iota.get_address().outputs(&address).await.unwrap();
    println!("The outputs of {:?} are {:?}", address, outputs);
}
