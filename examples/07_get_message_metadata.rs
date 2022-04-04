// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 07_get_message_metadata --release

use iota_client::{Client, Result};

/// In this example we will send a message and get the metadata for it

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let message = client.message().finish().await?;

    let metadata = client.get_message_metadata(&message.id()).await?;
    println!("Message metadata: {:?}", metadata);
    Ok(())
}
