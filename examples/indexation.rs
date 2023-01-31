// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example indexation --release

use iota_client::{bee_message::prelude::Payload, Client};

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

    let message = iota
        .message()
        .with_index("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await
        .unwrap();

    println!("Message ID: {}\n", message.id().0);

    let fetched_message_ids = iota.get_message().index("Hello").await.unwrap();
    println!("{fetched_message_ids:#?}\n");

    let fetched_msg = iota.get_message().data(&fetched_message_ids[0]).await.unwrap();
    println!("{fetched_msg:#?}\n");

    if let Payload::Indexation(payload) = fetched_msg.payload().as_ref().unwrap() {
        println!(
            "Data: {}",
            String::from_utf8(payload.data().to_vec()).expect("Found invalid UTF-8")
        );
    }
}
