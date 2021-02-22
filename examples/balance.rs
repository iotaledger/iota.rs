// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example balance --release
use iota::{Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get the balance of a known address
#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://localhost:14265") // Insert the node here
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();

    let address = "atoi1qzj86lzml2ktagye4mj0th6zymgka8lt96qre9yye0v8sawzmdu0ut90vm7";

    let balance = iota.get_address().balance(&address.into()).await.unwrap();
    println!("The balance of {:?} is {:?}", address, balance);

    let outputs = iota.get_address().outputs(&address.into()).await.unwrap();
    println!("The outputs of {:?} are {:?}", address, outputs);

    let output = iota.get_output(&outputs[0]).await.unwrap();
    println!("Output {:?}", output);

    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed =
        Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap()).unwrap();
    let seed_balance = iota.get_balance(&seed).finish().await.unwrap();
    println!("Account balance: {:?}i", seed_balance);
}
