// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 03_simple_block --release
//! In this example we will send a block without a payload.

use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    let client = Client::builder()
        .with_node(&node_url)?
        .with_pow_worker_count(1)
        .finish()?;

    let block = client.block().finish().await?;

    println!(
        "Empty block sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );
    Ok(())
}
