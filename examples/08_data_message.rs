// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 08_data_message --release

use iota_client::{bee_message::payload::Payload, Client, Result};

/// In this example we will send a message with a tagged data payload

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        // .with_permanode("http://18.196.167.57:8000/api/permanode/", None, None)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let message = client
        .message()
        .with_tag("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await?;

    println!(
        "Message sent https://explorer.iota.org/devnet/message/{}\n",
        message.id()
    );

    let fetched_msg = client.get_message_data(&message.id()).await?;
    println!("{:#?}\n", fetched_msg);

    if let Payload::TaggedData(payload) = fetched_msg.payload().as_ref().unwrap() {
        println!(
            "Data: {}",
            String::from_utf8(payload.data().to_vec()).expect("Found invalid UTF-8")
        );
    }
    Ok(())
}
