// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 3_send_message --release
use iota_client::{bee_message::payload::Payload, Client, Result};
use std::{fs::File, io::prelude::*, path::Path};

/// In this example we will send the signed transaction in a message

const SIGNED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // Get inputs and create transaction essence online
    let online_client = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_node_sync_disabled()
        .finish()
        .await?;

    let signed_transaction_payload = read_signed_transaction_from_file(SIGNED_TRANSACTION_FILE_NAME)?;

    // Send offline signed transaction online
    let message = online_client
        .message()
        .finish_message(Some(signed_transaction_payload))
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id()
    );
    Ok(())
}

fn read_signed_transaction_from_file<P: AsRef<Path>>(path: P) -> Result<Payload> {
    let mut file = File::open(&path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let signed_transaction_payload: Payload = serde_json::from_str(&data)?;
    Ok(signed_transaction_payload)
}
