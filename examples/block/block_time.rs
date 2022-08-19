// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example block_time --release

use iota_client::{Client, Result};

/// In this example we will send a block and return the time at which it got referenced by a milestone

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()?;

    let block = client.block().finish().await?;

    let block_id = block.id();
    println!("Block ID: {}", block_id);

    let _ = client.retry_until_included(&block_id, None, None).await?;

    let metadata = client.get_block_metadata(&block_id).await?;
    match metadata.referenced_by_milestone_index {
        Some(ms_index) => {
            let ms = client.get_milestone_by_index(ms_index).await?;
            println!(
                "Block got referenced by milestone {} at {}",
                ms_index,
                ms.essence().timestamp()
            );
        }
        _ => println!("Block is not referenced by a milestone"),
    }

    Ok(())
}
