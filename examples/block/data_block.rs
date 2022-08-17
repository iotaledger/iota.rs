// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example data_block --release

use iota_client::{block::payload::Payload, Client, Result};

/// In this example we will send a block with a tagged data payload

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    let client = Client::builder()
        .with_node(&node_url)?
        .with_node_sync_disabled()
        .finish()?;

    let block = client
        .block()
        .with_tag("Hello".as_bytes().to_vec())
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await?;

    println!(
        "Block sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    let fetched_block = client.get_block(&block.id()).await?;
    println!("{:#?}\n", fetched_block);

    if let Some(Payload::TaggedData(payload)) = fetched_block.payload() {
        println!(
            "Tag: {}",
            String::from_utf8(payload.tag().to_vec()).expect("Found invalid UTF-8")
        );
        println!(
            "Data: {}",
            String::from_utf8(payload.data().to_vec()).expect("Found invalid UTF-8")
        );
    }
    Ok(())
}
