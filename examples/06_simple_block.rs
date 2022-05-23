// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 06_simple_block --release

use iota_client::{Client, Result};

/// In this example we will send a block without a payload

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let block = client.block().finish().await?;

    println!(
        "Empty block sent: https://explorer.iota.org/devnet/block/{}",
        block.id()
    );
    Ok(())
}
