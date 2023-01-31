// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 04_get_balance --release

use iota_client::{Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will get the account balance of a known seed and the balance and outputs of a known address

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe") // Insert your node URL here
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();

    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap());

    let seed_balance = iota.get_balance(&seed).finish().await.unwrap();
    println!("Account balance: {seed_balance:?}i\n");

    let address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    let response = iota.get_address().balance(address).await.unwrap();
    println!("The balance of {:?} is {:?}i\n", address, response.balance);

    let outputs = iota.get_address().outputs(address, Default::default()).await.unwrap();

    println!("The outputs of address {address:?} are: {outputs:?}");
}
