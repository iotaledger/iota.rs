// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_payload --release
use iota::{Client, IndexationPayload, Payload};

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();
    let indexation_payload = IndexationPayload::new("index".as_bytes(), &"data".as_bytes().to_vec()).unwrap();
    let message = iota
        .message()
        .finish_message(Some(Payload::Indexation(Box::new(indexation_payload))))
        .await
        .unwrap();

    println!("MessageId {}", message.id().0);
}
