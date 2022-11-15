// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `GET /api/core/v2/info`.
//! Returns general information about the node.
//! Run: `cargo run --example node_api_core_get_info --release -- [NODE URL]`.

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
    let client = Client::builder()
        .with_node(&node_url)?
        .with_ignore_node_health()
        .finish()?;

    // Get node info.
    let info = client.get_info().await?;

    println!("{info:#?}");

    Ok(())
}
