// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `GET api/indexer/v1/outputs/alias`.
//! RUn: `cargo run --example node_api_indexer_get_alias_outputs --release -- [NODE URL] [ADDRESS]`.

use std::str::FromStr;

use iota_client::{bee_block::output::AliasId, node_api::indexer::query_parameters::QueryParameter, Client, Result};

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

    // Take the address from command line argument or use a default one.
    let address = std::env::args()
        .nth(2)
        .unwrap_or_else(|| String::from("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy"));

    // Get output ids of outputs that can be controlled by this address.
    let output_ids = client
        .alias_output_ids(vec![
            QueryParameter::Governor(address.to_string()),
            QueryParameter::StateController(address.to_string()),
        ])
        .await?;

    // Print the address output IDs.
    println!("Address output IDs {output_ids:#?}");

    // Get the outputs by their id.
    let outputs_responses = client.get_outputs(output_ids).await?;

    println!("Outputs: {outputs_responses:?}",);

    // Get an alias output by its AliasId.
    let alias_id = AliasId::from_str("0xd1d1e67e30effbc22671284531a5609b82969b030750468470faf03bf0afcb98")?;
    let output_id = client.alias_output_id(alias_id).await?;

    println!("Alias output: {output_id}");

    Ok(())
}
