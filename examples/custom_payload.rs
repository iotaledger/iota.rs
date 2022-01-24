// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_payload --release

use iota_client::{
    bee_message::payload::{Payload, TaggedPayload},
    Client,
};

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let indexation_payload =
        TaggedPayload::new("Your Index".as_bytes().to_vec(), "Your Data".as_bytes().to_vec()).unwrap();

    let message = iota
        .message()
        .finish_message(Some(Payload::Indexation(Box::new(indexation_payload))))
        .await
        .unwrap();

    println!("Message ID: {}", message.id());
}
