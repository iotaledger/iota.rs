// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 2_transaction_signing --release
use iota_client::{api::PreparedTransactionData, bee_message::payload::Payload, Client, Result, Seed};
extern crate dotenv;
use dotenv::dotenv;
use std::{
    env,
    fs::File,
    io::{prelude::*, BufWriter},
    path::Path,
};

/// In this example we will sign the prepared transaction

const PREPARED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/prepared_transaction.json";
const SIGNED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let iota_offline = Client::builder().with_offline_mode().finish().await?;

    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();
    let seed = Seed::from_bytes(&hex::decode(env::var("NONSECURE_USE_OF_DEVELOPMENT_SEED_1").unwrap())?);

    let prepared_transaction_data = read_prepared_transactiondata_from_file(PREPARED_TRANSACTION_FILE_NAME)?;

    // Sign prepared transaction offline
    let signed_transaction = iota_offline
        .message()
        .sign_transaction(
            prepared_transaction_data,
            Some(&seed),
            // indexes for the input addresses need to be in this range
            Some(0..100),
        )
        .await?;

    println!("Signed transaction");

    write_signed_transaction_to_file(SIGNED_TRANSACTION_FILE_NAME, signed_transaction)?;
    Ok(())
}

fn read_prepared_transactiondata_from_file<P: AsRef<Path>>(path: P) -> Result<PreparedTransactionData> {
    let mut file = File::open(&path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let prepared_transaction_data: PreparedTransactionData = serde_json::from_str(&data)?;
    Ok(prepared_transaction_data)
}

fn write_signed_transaction_to_file<P: AsRef<Path>>(path: P, signed_transaction: Payload) -> Result<()> {
    let jsonvalue = serde_json::to_value(&signed_transaction)?;
    let file = File::create(path)?;
    let bw = BufWriter::new(file);
    serde_json::to_writer_pretty(bw, &jsonvalue)?;
    Ok(())
}
