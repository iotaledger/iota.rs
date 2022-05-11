// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! `cargo run --example node_api_core_post_message_raw --release -- [NODE URL]`.

use iota_client::{
    bee_message::{parent::Parents, Message},
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let node = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "http://localhost:14265".to_string());
    let client = Client::builder()
        .with_node(&node)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let message = Message::build(Parents::new(client.get_tips().await?)?).finish()?;
    let message_id = client.post_message_raw(&message).await?;

    println!("{:?}", message_id);

    Ok(())
}
