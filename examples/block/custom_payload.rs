// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example custom_payload --release

use iota_client::{
    block::payload::{Payload, TaggedDataPayload},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()?;

    let tagged_data_payload = TaggedDataPayload::new("Your tag".as_bytes().to_vec(), "Your data".as_bytes().to_vec())?;

    let block = client
        .block()
        .finish_block(Some(Payload::TaggedData(Box::new(tagged_data_payload))))
        .await?;

    println!("Block ID: {}", block.id());
    Ok(())
}
