// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `GET api/indexer/v1/outputs/foundry/{foundryId}`.
//! Run: `cargo run --example node_api_indexer_get_foundry_output --release -- [NODE URL] [FOUNDRY ID]`.

use std::str::FromStr;

use iota_client::{block::output::FoundryId, Client, Result};

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
        .finish()?;

    // Take the foundry ID from command line argument or use a default one.
    let foundry_id = FoundryId::from_str(&std::env::args().nth(2).unwrap_or_else(|| {
        String::from("0x08db4db7643d768139d6f8ac3f9c9b7a82a245b619fa9f7c18fcd8f0f67e57abc20100000000")
    }))?;

    // Get the output ID by the foundry ID.
    let output_id = client.foundry_output_id(foundry_id).await?;

    println!("Foundry output ID: {output_id}");

    // Get the output by its ID.
    let output_response = client.get_output(&output_id).await?;

    println!("{output_response:#?}",);

    Ok(())
}
