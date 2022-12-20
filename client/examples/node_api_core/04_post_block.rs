// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `POST /api/core/v2/blocks`.
//! Submits a block as a JSON payload.
//! Run: `cargo run --example node_api_core_post_block --release -- [NODE URL]`.

use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses dotenv, which is not safe for use in production.
        dotenv::dotenv().ok();
        std::env::var("NODE_URL").unwrap()
    });

    // Create a client with that node.
    let client = Client::builder().with_node(&node_url)?.finish()?;

    // Create the block.
    let block = client.block().finish().await?;
    // Post the block.
    let block_id = client.post_block(&block).await?;

    println!("Posted: {block_id:?}");

    Ok(())
}
