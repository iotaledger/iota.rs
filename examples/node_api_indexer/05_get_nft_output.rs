// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `GET api/indexer/v1/outputs/nft/{nftId}`.
//! Run: `cargo run --example node_api_indexer_get_nft_output --release -- [NODE URL] [ADDRESS]`.

use std::str::FromStr;

use iota_client::{bee_block::output::NftId, Client, Result};

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
        // The node needs to have the indexer plugin enabled.
        .with_node(&node_url)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // Get an nft output by its NftId.
    let nft_id = NftId::from_str("0x649db5b14ee26d7eb91304cfeaa27cb661e1b05d366623be24d07955e0af6ce1")?;
    let output_id = client.nft_output_id(nft_id).await?;

    println!("Nft output: {output_id}");

    Ok(())
}
