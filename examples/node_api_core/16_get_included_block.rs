// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! `cargo run --example node_api_core_get_included_block --release -- [NODE URL]`.

use std::{env, str::FromStr};

use dotenv::dotenv;
use iota_client::{bee_block::payload::transaction::TransactionId, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Takes the node URL from command line argument or use localhost as default.
    let node = std::env::args().nth(1).unwrap_or_else(|| {
        dotenv().ok();
        env::var("NODE_URL").unwrap()
    });
    // Creates a client instance with that node.
    let client = Client::builder()
        .with_node(&node)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // Transactions get pruned from the node after some time, replace with a new TransactionId.
    let transaction_id = TransactionId::from_str("0xb66fd384cb5755668f1890ea2e41d699db9cf32f3bc422ad3c24ffeb9c7f01d0")?;
    // Sends the request.
    let block = client.get_included_block(&transaction_id).await?;

    // Prints the response.
    println!("{:#?}", block);

    Ok(())
}
