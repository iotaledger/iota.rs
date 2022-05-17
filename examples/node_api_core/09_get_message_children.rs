// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! `cargo run --example node_api_core_get_get_message_children --release -- [NODE URL]`.

use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Takes the node URL from command line argument or use localhost as default.
    let node = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "http://localhost:14265".to_string());
    // Creates a client instance with that node.
    let client = Client::builder()
        .with_node(&node)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // Fetches a tip message ID from the node.
    let tip_message_id = client.get_tips().await?[0];
    // Tips have, by definition, no children so this gets the children of a tip parent.
    let tip = client.get_message(&tip_message_id).await?;
    let message_id = tip.parents()[0];
    // Sends the request.
    let message_children = client.get_message_children(&message_id).await?;

    // Prints the response.
    println!("{:?}", message_children);

    Ok(())
}
