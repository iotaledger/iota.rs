// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example dust --release
use iota::{Client, Seed};

#[macro_use]
extern crate dotenv_codegen;

/// In this example, we send a dust allowance output

#[tokio::main]
async fn main() {
    let iota = Client::build() // Crate a client instance builder
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .unwrap();

    let seed = Seed::from_ed25519_bytes(&hex::decode(dotenv!("seed")).unwrap()).unwrap();

    let message_id = iota
        .send()
        .with_seed(&seed)
        // Insert the output address and amount to spent. The amount cannot be zero.
        .with_dust_allowance_output(
            &"iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4".into(),
            1_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "First transaction sent: http://127.0.0.1:14265/api/v1/messages/{}",
        message_id
    );
}
