// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 07_get_block_metadata --release

use iota_client::{Client, Result};

/// In this example we will send a block and get the metadata for it

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let block = client.block().finish().await?;

    let metadata = client.get_block_metadata(&block.id()).await?;
    println!("Block metadata: {:?}", metadata);
    Ok(())
}
