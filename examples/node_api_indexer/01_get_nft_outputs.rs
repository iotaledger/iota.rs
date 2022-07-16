// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! `cargo run --example node_api_indexer_get_nft_outputs --release -- [NODE URL]`.

use std::{env, str::FromStr};

use dotenv::dotenv;
use iota_client::{block::output::NftId, node_api::indexer::query_parameters::QueryParameter, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Takes the node URL from command line argument or use localhost as default.
    let node = std::env::args().nth(1).unwrap_or_else(|| {
        dotenv().ok();
        env::var("NODE_URL").unwrap()
    });
    // Creates a client instance with that node.
    let client = Client::builder()
        // The nodes needs to have the indexer plugin enabled.
        .with_node(&node)?
        .with_node_sync_disabled()
        .finish()?;

    let address = "rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy";

    // Get output ids of outputs that can be controlled by this address without further unlock constraints.
    let output_ids = client
        .nft_output_ids(vec![
            QueryParameter::Address(address.to_string()),
            QueryParameter::HasExpirationCondition(false),
            QueryParameter::HasTimelockCondition(false),
            QueryParameter::HasStorageReturnCondition(false),
        ])
        .await?;

    println!("Address output_ids {:?}", output_ids);

    // Get the outputs by their id.
    let outputs_responses = client.get_outputs(output_ids).await?;
    println!("Nft outputs: {outputs_responses:?}",);

    // Get an nft output by its NftId.
    let nft_id = NftId::from_str("0x649db5b14ee26d7eb91304cfeaa27cb661e1b05d366623be24d07955e0af6ce1")?;
    let output_id = client.nft_output_id(nft_id).await?;
    println!("Nft output: {output_id}");

    Ok(())
}
