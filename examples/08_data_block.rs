// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 08_data_block --release

use iota_client::{bee_block::payload::Payload, Client, Result};

/// In this example we will send a block with a tagged data payload

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        // .with_permanode("http://18.196.167.57:8000/api/permanode/", None, None)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let block = client
        .block()
        .with_tag("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await?;

    println!("Block sent https://explorer.iota.org/devnet/block/{}\n", block.id());

    let fetched_block = client.get_block(&block.id()).await?;
    println!("{:#?}\n", fetched_block);

    if let Payload::TaggedData(payload) = fetched_block.payload().as_ref().unwrap() {
        println!(
            "Data: {}",
            String::from_utf8(payload.data().to_vec()).expect("Found invalid UTF-8")
        );
    }
    Ok(())
}
