// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 04_get_address_outputs --release

use iota_client::{node_api::indexer::query_parameters::QueryParameter, Client, Result};

/// In this example we will get the outputs of a known address

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids = client
        .output_ids(vec![
            QueryParameter::Address(address.to_string()),
            QueryParameter::HasExpirationCondition(false),
            QueryParameter::HasTimelockCondition(false),
            QueryParameter::HasStorageDepositReturnCondition(false),
        ])
        .await?;

    println!("Address output_ids {:?}", output_ids);

    // Get the outputs by their id
    let outputs_responses = client.get_outputs(output_ids).await?;
    println!("Outputs: {:?}", outputs_responses);
    Ok(())
}
