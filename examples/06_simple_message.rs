// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 06_simple_message --release

use iota_client::{Client, Result};

/// In this example we will send a message without a payload

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let message = client.message().finish().await?;

    println!(
        "Empty message sent: https://explorer.iota.org/devnet/message/{}",
        message.id()
    );
    Ok(())
}
