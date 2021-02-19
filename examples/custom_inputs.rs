// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_inputs --release
use iota::{Client, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::env;
/// In this example, we send 1_000_000 tokens to atoi1qzj86lzml2ktagye4mj0th6zymgka8lt96qre9yye0v8sawzmdu0ut90vm7
/// This address belongs to the first seed in .env.example

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    // Insert your seed. Since the output amount cannot be zero. The seed must contain non-zero balance.
    // First address from the seed below is iot1qxt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupxgecea4
    println!("This example uses dotenv, which is not safe for use in production.");
    dotenv().ok();
    let seed =
        Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap()).unwrap()).unwrap();

    let address = iota
        .find_addresses(&seed)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();
    println!("{:?}", address[0]);
    let outputs = iota.get_address().outputs(&address[0]).await.unwrap();
    println!("{:?}", outputs);

    let message = iota
        .message()
        .with_seed(&seed)
        .with_input(outputs[0].clone())
        // .with_input_range(20..25)
        .with_output(
            &"atoi1qzj86lzml2ktagye4mj0th6zymgka8lt96qre9yye0v8sawzmdu0ut90vm7".into(),
            1_000_000,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    println!(
        "Transaction sent: https://explorer.iota.org/chrysalis/message/{}",
        message.id().0
    );
}
