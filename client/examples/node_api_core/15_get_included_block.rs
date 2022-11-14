// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `GET /api/core/v2/transactions/{transactionId}/included-block`.
//! Returns the included block, as JSON, of a transaction.
//! Run: `cargo run --example node_api_core_get_included_block --release -- [NODE URL]`.

use std::str::FromStr;

use iota_client::{block::payload::transaction::TransactionId, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses dotenv, which is not safe for use in production.
        dotenv::dotenv().ok();
        std::env::var("NODE_URL").unwrap()
    });

    // Create a client with that node.
    let client = Client::builder().with_node(&node_url)?.finish()?;

    // Transactions get pruned from the node after some time, replace with a new TransactionId.
    let transaction_id = TransactionId::from_str("0xb66fd384cb5755668f1890ea2e41d699db9cf32f3bc422ad3c24ffeb9c7f01d0")?;
    // Send the request.
    let block = client.get_included_block(&transaction_id).await?;

    println!("{block:#?}");

    Ok(())
}
