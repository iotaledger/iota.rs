// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `GET /api/core/v2/tips`.
//! Returns tips that are ideal for attaching a block.
//! Run: `cargo run --example node_api_core_get_tips --release -- [NODE URL]`.

use std::env;

use dotenv::dotenv;
use iota_client::{Client, Result};

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

    // Sends the request.
    let tips = client.get_tips().await?;

    // Prints the response.
    println!("{:#?}", tips);

    Ok(())
}
