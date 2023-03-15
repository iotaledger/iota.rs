// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `GET api/indexer/v1/outputs/basic`.
//! Run: `cargo run --example node_api_indexer_get_random_basic_outputs --release -- [NODE URL]`.

use iota_client::{node_api::indexer::query_parameters::QueryParameter, Client, Result};

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

    // Get a single page with random output IDs by providing only `QueryParameter::Cursor(_)`.
    let output_ids_response = client
        .basic_output_ids(vec![QueryParameter::Cursor(String::new())])
        .await?;

    println!("Address output IDs {output_ids_response:#?}");

    // Get the outputs by their IDs.
    let outputs_responses = client.get_outputs(output_ids_response.items).await?;

    println!("Basic outputs: {outputs_responses:#?}");

    Ok(())
}
