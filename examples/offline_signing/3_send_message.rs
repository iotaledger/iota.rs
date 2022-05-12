// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we send the signed transaction in a message.
//! `cargo run --example 3_send_message --release`.

use std::{fs::File, io::prelude::*, path::Path};

use iota_client::{
    api::{verify_semantic, PreparedTransactionData},
    bee_message::{payload::Payload, semantic::ConflictReason},
    Client, Error, Result,
};

const PREPARED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/prepared_transaction.json";
const SIGNED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // Creates a client instance.
    let online_client = Client::builder()
        // Insert your node URL here.
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let signed_transaction_payload = read_signed_transaction_from_file(SIGNED_TRANSACTION_FILE_NAME)?;

    // TODO @thibault-martinez: I don't like that we have to refetch the prepared transaction. Will revisit later.
    if let Payload::Transaction(ref signed_transaction_payload) = signed_transaction_payload {
        let prepared_transaction = read_prepared_transaction_from_file(PREPARED_TRANSACTION_FILE_NAME)?;
        let (local_time, milestone_index) = online_client.get_time_and_milestone_checked().await?;

        let conflict = verify_semantic(
            &prepared_transaction.inputs_data,
            signed_transaction_payload,
            milestone_index,
            local_time,
        )?;

        if conflict != ConflictReason::None {
            return Err(Error::TransactionSemantic(conflict));
        }
    } else {
        panic!("Payload should be a transaction");
    }

    // Sends offline signed transaction online.
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

fn read_prepared_transaction_from_file<P: AsRef<Path>>(path: P) -> Result<PreparedTransactionData> {
    let mut file = File::open(&path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    Ok(serde_json::from_str(&json)?)
}

fn read_signed_transaction_from_file<P: AsRef<Path>>(path: P) -> Result<Payload> {
    let mut file = File::open(&path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    Ok(serde_json::from_str(&json)?)
}
