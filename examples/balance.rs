// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example balance --release
use iota::Client;
use std::convert::TryInto;

/// In this example we will get the balance of a known address
#[tokio::main]
async fn main() {
    let iota = Client::build() // Crate a client instance builder
        .with_node("http://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .unwrap();

    let address = "atoi1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxtmtev5";

    let balance = iota.get_address().balance(&address.try_into().unwrap()).await.unwrap();
    println!("The balance of {:?} is {:?}", address, balance);

    let outputs = iota.get_address().outputs(&address.try_into().unwrap()).await.unwrap();
    println!("The outputs of {:?} are {:?}", address, outputs);
}
