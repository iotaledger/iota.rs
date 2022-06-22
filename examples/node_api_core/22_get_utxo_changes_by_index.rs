// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! `cargo run --example node_api_core_get_utxo_changes_by_index --release -- [NODE URL]`.

use std::env;

use dotenv::dotenv;
use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Takes the node URL from command line argument or use localhost as default.
    let node = std::env::args().nth(1).unwrap_or_else(|| {
        dotenv().ok();
        env::var("NODE_URL").unwrap()
    });
    // Creates a client instance with that node.
    let client = Client::builder()
        .with_node(&node)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // Fetches the latest milestone index from the node.
    let info = client.get_info().await?;
    let milestone_index = info.node_info.status.latest_milestone.index;
    // Sends the request.
    let utxo_changes = client.get_utxo_changes_by_index(milestone_index).await?;

    // Prints the response.
    println!("{:#?}", utxo_changes);

    Ok(())
}
