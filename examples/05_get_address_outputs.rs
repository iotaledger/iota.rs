// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 05_get_address_outputs --release

use iota_client::{node_api::indexer_api::query_parameters::QueryParameter, Client, Result};

/// In this example we will get the outputs of a known address

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    let output_ids =
        iota_client::node_api::indexer_api::routes::output_ids(&client, vec![QueryParameter::Address(address.into())])
            .await?;

    println!("Address output_ids {:?}", output_ids);

    let mut outputs = Vec::new();
    for output_id in &output_ids {
        let output = client.get_output(output_id).await?;
        outputs.push(output);
    }
    println!("Outputs: {:?}", outputs);
    Ok(())
}
