// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `POST /api/core/v2/blocks`.
//! Submits a block as a JSON payload.
//! Run: `cargo run --example node_api_core_post_block --release -- [NODE URL]`.

use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_block::{parent::Parents, Block},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Takes the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses dotenv, which is not safe for use in production.
        dotenv().ok();
        env::var("NODE_URL").unwrap()
    });

    // Creates a client instance with that node.
    let client = Client::builder()
        .with_node(&node_url)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // Creates a block.
    let block = Block::build(Parents::new(client.get_tips().await?)?).finish()?;
    // Sends the request.
    let block_id = client.post_block(&block).await?;

    // Prints the response.
    println!("{:?}", block_id);

    Ok(())
}
