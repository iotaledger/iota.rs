// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! `cargo run --example node_api_indexer_get_alias_outputs --release -- [NODE URL]`.

use std::str::FromStr;

use bee_block::output::AliasId;
use iota_client::{node_api::indexer::query_parameters::QueryParameter, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Takes the node URL from command line argument or use localhost as default.
    let node = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "http://localhost:14265".to_string());
    // Creates a client instance with that node.
    let client = Client::builder()
        // The nodes needs to have the indexer plugin enabled.
        .with_node(&node)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let address = "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy";

    // Get output ids of outputs that can be controlled by this address.
    let output_ids = client
        .alias_output_ids(vec![
            QueryParameter::Governor(address.to_string()),
            QueryParameter::StateController(address.to_string()),
        ])
        .await?;

    println!("Address output_ids {:?}", output_ids);

    // Get the outputs by their id.
    let outputs_responses = client.get_outputs(output_ids).await?;
    println!("Outputs: {outputs_responses:?}",);

    // Get an alias output by its AliasId.
    let alias_id = AliasId::from_str("0xd1d1e67e30effbc22671284531a5609b82969b030750468470faf03bf0afcb98")?;
    let output_id = client.alias_output_id(alias_id).await?;
    println!("Alias output: {output_id}");

    Ok(())
}
