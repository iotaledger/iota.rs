// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_payload --release

use iota_client::{
    bee_message::payload::{Payload, TaggedDataPayload},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_default_logger()?
        .finish()
        .await
        .unwrap();

    let tagged_data_payload =
        TaggedDataPayload::new("Your tag".as_bytes().to_vec(), "Your data".as_bytes().to_vec()).unwrap();

    let message = iota
        .message()
        .finish_message(Some(Payload::TaggedData(Box::new(tagged_data_payload))))
        .await
        .unwrap();

    println!("Message ID: {}", message.id());
    Ok(())
}
