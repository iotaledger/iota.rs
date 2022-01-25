// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 08_data_message --release

use iota_client::{Client, Result};

/// In this example we will send a message without a payload

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("http://localhost:14265")?
        // .with_permanode("http://18.196.167.57:8000/api/permanode/", None, None)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let message = iota
        .message()
        .with_tag("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await?;

    println!(
        "Message sent https://explorer.iota.org/devnet/message/{}\n",
        message.id()
    );

    let fetched_message_ids = iota.get_message().index("Hello").await.unwrap();
    println!("Messages with Hello index: {:?}", fetched_message_ids);
    Ok(())
}
