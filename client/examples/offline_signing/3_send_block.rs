// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we send the signed transaction in a block.
//! `cargo run --example 3_send_block --release`.

use std::{fs::File, io::prelude::*, path::Path};

use iota_client::{
    api::{verify_semantic, SignedTransactionData, SignedTransactionDataDto},
    block::{payload::Payload, semantic::ConflictReason},
    Client, Error, Result,
};

const SIGNED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance.
    let online_client = Client::builder()
        // Insert your node URL in the .env.
        .with_node(&node_url)?
        .finish()?;

    let signed_transaction_payload = read_signed_transaction_from_file(SIGNED_TRANSACTION_FILE_NAME)?;

    let current_time = online_client.get_time_checked().await?;

    let conflict = verify_semantic(
        &signed_transaction_payload.inputs_data,
        &signed_transaction_payload.transaction_payload,
        current_time,
    )?;

    if conflict != ConflictReason::None {
        return Err(Error::TransactionSemantic(conflict));
    }

    // Sends the offline signed transaction online.
    let block = online_client
        .block()
        .finish_block(Some(Payload::Transaction(Box::new(
            signed_transaction_payload.transaction_payload,
        ))))
        .await?;

    println!(
        "Transaction sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    Ok(())
}

fn read_signed_transaction_from_file<P: AsRef<Path>>(path: P) -> Result<SignedTransactionData> {
    let mut file = File::open(&path).unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();

    let dto = serde_json::from_str::<SignedTransactionDataDto>(&json)?;

    Ok(SignedTransactionData::try_from_dto_unverified(&dto)?)
}
