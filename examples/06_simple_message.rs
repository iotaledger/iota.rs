// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 06_simple_message --release

use iota_client::{Client, Result};

/// In this example we will send a message without a payload

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")?
        .finish()
        .await?;

    let message = iota.message().finish().await?;

    println!(
        "Empty message sent: https://explorer.iota.org/devnet/message/{}",
        message.id().0
    );
    Ok(())
}
