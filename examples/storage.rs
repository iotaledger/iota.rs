// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example storage --release
use iota::{
    storage::{sqlite, StorageAdapter},
    Client,
};

#[tokio::main]
async fn main() {
    let iota = Client::builder() // Crate a client instance builder
        .with_node("https://api.hornet-0.testnet.chrysalis2.com") // Insert the node here
        .unwrap()
        .finish()
        .await
        .unwrap();

    let message = iota
        .message()
        .with_index("Helloo")
        .with_data("Tangle".to_string().as_bytes().to_vec())
        .finish()
        .await
        .unwrap();

    println!("MessageId {}", message.id().0);

    let path = "./the-storage-path.db";
    let mut storage_adapter = sqlite::SqliteStorageAdapter::new(path, "table_name").unwrap();
    storage_adapter
        .set("message_id", message.id().0.to_string())
        .await
        .unwrap();
    let account = storage_adapter.get("message_id").await.unwrap();
    println!("{:?}", account);
}
