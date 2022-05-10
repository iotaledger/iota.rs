// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we will get information about the node.
//! `cargo run --example node_core_api_get_info --release`.

use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let node = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "http://localhost:14265".to_string());
    let client = Client::builder()
        .with_node(&node)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let info = client.get_info().await?;

    println!("{:?}", info);

    Ok(())
}
