// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 04_get_block_metadata --release

use std::env;

use dotenv::dotenv;
use iota_client::{Client, Result};

/// In this example we will send a block and get the metadata for it

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let node_url = env::var("NODE_URL").unwrap();

    let client = Client::builder()
        .with_node(&node_url)?
        .with_node_sync_disabled()
        .finish()?;

    let block = client.block().finish().await?;

    let metadata = client.get_block_metadata(&block.id()).await?;
    println!("Block metadata: {:?}", metadata);
    Ok(())
}
