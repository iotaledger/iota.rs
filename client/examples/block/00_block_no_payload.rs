// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! This example sends a block with no payload.
//! Run: `cargo run --example block_no_payload --release -- [NODE URL]`.

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

    // Create and send the block.
    let block = client.block().finish().await?;

    println!("{block:#?}");

    println!(
        "Block with no payload sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}
