// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! `cargo run --example node_api_indexer_get_foundry_outputs --release -- [NODE URL]`.

use std::str::FromStr;

use bee_block::output::FoundryId;
use iota_client::{node_api::indexer::query_parameters::QueryParameter, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Takes the node URL from command line argument or use localhost as default.
    let node = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "http://localhost:14265".to_string());
    // Creates a client instance with that node.
    let client = Client::builder()
        // The nodes needs to have the indexer plugin enabled
        .with_node(&node)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let alias_address = "rms1ppdr9w5wmyg7phcd7q9exv2kvnu5rnwafftsehjpfwd6zxn83938xw83dtr";

    // Get output ids of foundry outputs that can be controlled by this address
    let output_ids = client
        .foundry_output_ids(vec![QueryParameter::AliasAddress(alias_address.to_string())])
        .await?;

    println!("Address output_ids {:?}", output_ids);

    // Get the outputs by their id
    let outputs_responses = client.get_outputs(output_ids).await?;
    println!("Outputs: {outputs_responses:?}",);

    // Get an foundry output by its FoundryId
    let foundry_id =
        FoundryId::from_str("0x085a32ba8ed911e0df0df00b93315664f941cddd4a570cde414b9ba11a678962730100000000")?;
    let output_id = client.foundry_output_id(foundry_id).await?;
    println!("Foundry output: {output_id}");

    Ok(())
}
