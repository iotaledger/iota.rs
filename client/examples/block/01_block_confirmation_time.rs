// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block and returns the time at which it got confirmed.
//! Run: `cargo run --example block_confirmation_time --release -- [NODE URL]`.

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

    // Create and send a block.
    let block = client.block().finish().await?;
    let block_id = block.id();

    println!("{block:#?}");

    // Try to check if the block has been confirmed.
    let _ = client.retry_until_included(&block_id, None, None).await?;

    // Get the block metadata.
    let metadata = client.get_block_metadata(&block_id).await?;

    if let Some(ms_index) = metadata.referenced_by_milestone_index {
        let ms = client.get_milestone_by_index(ms_index).await?;
        println!(
            "Block {block_id} got confirmed by milestone {ms_index} at timestamp {}.",
            ms.essence().timestamp()
        );
    } else {
        println!("Block {block_id} is not confirmed.")
    }

    Ok(())
}
