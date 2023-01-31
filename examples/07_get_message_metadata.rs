// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 07_get_message_metadata --release

use iota_client::{Client, Result};

/// In this example we will send a message and get the metadata for it

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")?
        .finish()
        .await?;

    let message = iota.message().finish().await?;

    let metadata = iota.get_message().metadata(&message.id().0).await?;
    println!("Message metadata: {metadata:?}");
    Ok(())
}
