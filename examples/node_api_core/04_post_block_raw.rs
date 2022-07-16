// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! `cargo run --example node_api_core_post_block_raw --release -- [NODE URL]`.

use std::env;

use dotenv::dotenv;
use iota_client::{
    block::{parent::Parents, Block},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Takes the node URL from command line argument or use localhost as default.
    let node = std::env::args().nth(1).unwrap_or_else(|| {
        dotenv().ok();
        env::var("NODE_URL").unwrap()
    });
    // Creates a client instance with that node.
    let client = Client::builder().with_node(&node)?.with_node_sync_disabled().finish()?;

    // Creates a block.
    let block = Block::build(Parents::new(client.get_tips().await?)?).finish()?;
    // Sends the request.
    let block_id = client.post_block_raw(&block).await?;

    // Prints the response.
    println!("{:?}", block_id);

    Ok(())
}
