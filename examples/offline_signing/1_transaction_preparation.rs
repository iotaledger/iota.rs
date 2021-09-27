// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 1_transaction_preparation --release
use iota_client::{api::PreparedTransactionData, Client, Result};
use std::{
    fs::File,
    io::{prelude::*, BufWriter},
    path::Path,
};

/// In this example we will get inputs and prepare a transaction

const ADDRESS_FILE_NAME: &str = "examples/offline_signing/addresses.json";
const PREPARED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/prepared_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    // Address to which we want to send the amount
    let address = "atoi1qruzprxum2934lr3p77t96pzlecxv8pjzvtjrzdcgh2f5exa22n6gek0qdq";
    let amount = 1_000_000;

    // Get inputs and create transaction essence online
    let iota_online = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")? // Insert your node URL here
        .finish()
        .await?;

    let addresses = read_addresses_from_file(ADDRESS_FILE_NAME)?;

    let inputs = iota_online.find_inputs(addresses, amount).await?;

    // Prepare transaction
    let mut transaction_builder = iota_online.message();
    for input in inputs {
        transaction_builder = transaction_builder.with_input(input);
    }
    let prepared_transaction_data = transaction_builder
        .with_output(address, amount)?
        .prepare_transaction()
        .await?;

    println!("Prepared transaction sending {} to {}", amount, address);

    write_transaction_to_file(PREPARED_TRANSACTION_FILE_NAME, prepared_transaction_data)?;

    Ok(())
}

fn read_addresses_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<String>> {
    let mut file = File::open(&path)?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;
    let addresses: Vec<String> = serde_json::from_str(&data)?;
    Ok(addresses)
}

fn write_transaction_to_file<P: AsRef<Path>>(
    path: P,
    prepared_transaction_data: PreparedTransactionData,
) -> Result<()> {
    let jsonvalue = serde_json::to_value(&prepared_transaction_data)?;
    let file = File::create(path)?;
    let bw = BufWriter::new(file);
    serde_json::to_writer_pretty(bw, &jsonvalue)?;
    Ok(())
}
