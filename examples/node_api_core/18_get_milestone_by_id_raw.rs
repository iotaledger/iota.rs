// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Calls `GET /api/core/v2/milestones/{milestoneId}`.
//! Returns milestone data as raw bytes by its identifier.
//! Run: `cargo run --example node_api_core_get_milestone_by_id_raw --release -- [NODE URL]`.

use std::{env, str::FromStr};

use dotenv::dotenv;
use iota_client::{bee_block::payload::milestone::MilestoneId, Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Takes the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses dotenv, which is not safe for use in production.
        dotenv().ok();
        env::var("NODE_URL").unwrap()
    });

    // Creates a client instance with that node.
    let client = Client::builder()
        .with_node(&node_url)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // Fetches the latest milestone ID from the node.
    let info = client.get_info().await?;
    let milestone_id = MilestoneId::from_str(&info.node_info.status.latest_milestone.milestone_id)?;
    // Sends the request.
    let milestone = client.get_milestone_by_id_raw(&milestone_id).await?;

    // Prints the response.
    println!("{:?}", milestone);

    Ok(())
}
