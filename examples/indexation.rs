// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example indexation --release
use iota::{Client, Payload};

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        // .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        // .unwrap()
        .with_node("http://localhost:14265")
        .unwrap()
        .with_node_sync_disabled()
        .finish()
        .await
        .unwrap();

    let message = iota
        .send()
        .with_index("Hello")
        .with_data("Tangle".to_string().as_bytes().to_vec())
        .finish()
        .await
        .unwrap();

    println!("MessageId {}", message.id().0);

    let fetched_message_ids = iota.get_message().index(&"Hello").await.unwrap();

    println!("{:#?}", fetched_message_ids);

    let fetched_msg = iota.get_message().data(&fetched_message_ids[0]).await.unwrap();

    println!("{:#?}", fetched_msg);
    if let Payload::Indexation(i) = fetched_msg.payload().as_ref().unwrap() {
        println!(
            "Data: {}",
            String::from_utf8(i.data().to_vec()).expect("Found invalid UTF-8")
        );
    }
}
