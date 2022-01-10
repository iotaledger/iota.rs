// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 05_get_address_outputs --release

use iota_client::{bee_message::input::UtxoInput, bee_message::output::OutputId, Client, Result};
use std::str::FromStr;

/// In this example we will get the outputs of a known address

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    let address_outputs_response = client
        .get_address()
        .outputs_response(address, Default::default())
        .await
        .unwrap();

    println!("Address outputs response {:?}", address_outputs_response);

    let mut outputs = Vec::new();
    for output in &address_outputs_response.output_ids {
        let output = client.get_output(&UtxoInput::from(OutputId::from_str(output)?)).await?;
        outputs.push(output);
    }
    println!("Outputs: {:?}", outputs);
    Ok(())
}
