// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `POST /api/core/v2/blocks`.
//! Submits a block as raw bytes.
//! Run: `cargo run --example node_api_core_post_block_raw --release -- [NODE URL]`.

use iota_client::{
    block::{parent::Parents, Block},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses dotenv, which is not safe for use in production.
        dotenv::dotenv().ok();
        std::env::var("NODE_URL").unwrap()
    });

    // Create a client with that node.
    let client = Client::builder()
        .with_node(&node_url)?
        .with_node_sync_disabled()
        .finish()?;

    let min_pow_score = client.get_min_pow_score()?;

    // Get parents for the block.
    let parents = Parents::new(client.get_tips().await?)?;
    // Create the block.
    let block = Block::build(parents).finish(min_pow_score)?;
    // Post the block as raw bytes.
    let block_id = client.post_block_raw(&block).await?;

    println!("Posted: {block_id:?}");

    Ok(())
}
