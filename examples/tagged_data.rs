// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example tagged_data --release

use iota_client::{bee_message::payload::Payload, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_node_sync_disabled()
        .with_default_logger()?
        .finish()
        .await?;

    let message = iota
        .message()
        .with_tag("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await?;

    println!("Message ID: {}\n", message.id());

    let fetched_msg = iota.get_message().data(&message.id()).await?;
    println!("{:#?}\n", fetched_msg);

    if let Payload::TaggedData(payload) = fetched_msg.payload().as_ref().unwrap() {
        println!(
            "Data: {}",
            String::from_utf8(payload.data().to_vec()).expect("Found invalid UTF-8")
        );
    }
    Ok(())
}
