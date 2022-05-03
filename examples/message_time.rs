// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example message_time --release

use iota_client::Client;

#[tokio::main]
async fn main() {
    // Create a client instance
    let client = Client::builder()
        .with_node("http://localhost:14265") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let message = client
        .message()
        .with_tag("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await
        .unwrap();

    let message_id = message.id();
    println!("Message ID: {}", message_id);

    let _ = client.retry_until_included(&message_id, None, None).await.unwrap();

    let metadata = client.get_message_metadata(&message_id).await.unwrap();
    match metadata.referenced_by_milestone_index {
        Some(ms_index) => {
            let ms = client.get_milestone_by_milestone_index(ms_index).await.unwrap();
            println!("Message got referenced by milestone {} at {}", ms_index, ms.0.timestamp);
        }
        _ => println!("Message is not referenced by a milestone"),
    }
}
