// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example message_time --release
use iota::{Client, MessageId};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("https://api.lb-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let message = iota
        .message()
        .with_index("Hello")
        .with_data("Tangle".to_string().as_bytes().to_vec())
        .finish()
        .await
        .unwrap();
    let message_id = message.id().0;
    println!("MessageId {}", message_id);

    reattach_promote_until_confirmed(&message_id, &iota).await;

    let metadata = iota.get_message().metadata(&message_id).await.unwrap();
    match metadata.referenced_by_milestone_index {
        Some(ms_index) => {
            let ms = iota.get_milestone(ms_index).await.unwrap();
            println!("Message got referenced by milestone {} at {}", ms_index, ms.timestamp);
        }
        _ => println!("Message is not referenced by a milestone"),
    }
}

async fn reattach_promote_until_confirmed(message_id: &MessageId, iota: &Client) {
    while let Ok(metadata) = iota.get_message().metadata(&message_id).await {
        if metadata.referenced_by_milestone_index.is_some() {
            break;
        } else if let Ok(msg_id) = iota.reattach(&message_id).await {
            println!("Reattached or promoted {}", msg_id.0);
        }
        sleep(Duration::from_secs(5)).await;
    }
}
