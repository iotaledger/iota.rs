// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_parent --release
use iota::{Client, MessageId};
use std::str::FromStr;
/// In this example, we define a custom message parent, can be used for promoting

#[tokio::main]
async fn main() {
    let iota = Client::build() // Crate a client instance builder
        .with_node("http://0.0.0.0:14265") // Insert the node here
        .unwrap()
        .finish()
        .unwrap();
    let custom_parent =
        MessageId::from_str("b5634e05a7c665d7f87330a53633f001a5d1d96b346dc98dc225c4d6c204f23b").unwrap();

    let message = iota.send().with_parent(custom_parent).finish().await.unwrap();

    println!(
        "Empty message sent: https://explorer.iota.org/chrysalis/message/{}",
        message.id().0
    );
}
