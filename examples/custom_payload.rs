// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_payload --release

use iota_client::{
    bee_message::prelude::{IndexationPayload, Payload},
    Client,
};

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let indexation_payload = IndexationPayload::new("Your Index".as_bytes(), "Your Data".as_bytes()).unwrap();

    let message = iota
        .message()
        .finish_message(Some(Payload::Indexation(Box::new(indexation_payload))))
        .await
        .unwrap();

    println!("Message ID: {}", message.id().0);
}
