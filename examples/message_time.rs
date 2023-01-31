// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example message_time --release

use iota_client::Client;

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe") // Insert your node URL here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let message = iota
        .message()
        .with_index("Hello")
        .with_data("Tangle".as_bytes().to_vec())
        .finish()
        .await
        .unwrap();

    let message_id = message.id().0;
    println!("Message ID: {message_id}");

    let _ = iota.retry_until_included(&message_id, None, None).await.unwrap();

    let metadata = iota.get_message().metadata(&message_id).await.unwrap();
    match metadata.referenced_by_milestone_index {
        Some(ms_index) => {
            let ms = iota.get_milestone(ms_index).await.unwrap();
            println!("Message got referenced by milestone {} at {}", ms_index, ms.timestamp);
        }
        _ => println!("Message is not referenced by a milestone"),
    }
}
