// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example storage --features=storage --release

use iota_client::{
    storage::{sqlite, StorageAdapter},
    Client,
};

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

    println!("Message ID: {}", message.id().0);

    let path = "./the-storage-path.db";
    let mut storage_adapter = sqlite::SqliteStorageAdapter::new(path, "table_name").unwrap();

    storage_adapter
        .set("message_id", message.id().0.to_string())
        .await
        .unwrap();

    let message_id = storage_adapter.get("message_id").await.unwrap();
    println!("Message ID from storage: {message_id:?}");
}
