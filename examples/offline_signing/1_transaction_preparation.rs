// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 1_transaction_preparation --release
use iota_client::{
    api::{AddressIndexRecorder, ClientMessageBuilder},
    bee_message::{constants::INPUT_OUTPUT_COUNT_MAX, input::UtxoInput, payload::transaction::Essence},
    Client, Result,
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{prelude::*, BufWriter},
    path::Path,
};
/// In this example we will get inputs and prepare a transaction

const ADDRESS_FILE_NAME: &str = "examples/offline_signing/addresses.json";
const PREPARED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/prepared_transaction.json";
const DUST_THRESHOLD: u64 = 1_000_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PreparedTransactionData {
    essence: Essence,
    address_index_recorders: Vec<AddressIndexRecorder>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Address to which we want to send the amount
    let address = "atoi1qruzprxum2934lr3p77t96pzlecxv8pjzvtjrzdcgh2f5exa22n6gek0qdq";
    let amount = 1_000_000;

    // Get inputs and create transaction essence online
    let iota_online = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com")? // Insert your node URL here
        .finish()
        .await?;

    let addresses = read_addresses_from_file(ADDRESS_FILE_NAME)?;

    // Get outputs from node and select inputs
    let mut available_outputs = Vec::new();
    for address in addresses {
        available_outputs.extend_from_slice(&iota_online.get_address().outputs(&address, Default::default()).await?);
    }
    println!("Available outputs: {:?}", available_outputs.len());
    let inputs = get_inputs(&iota_online, available_outputs, amount).await?;

    // Prepare transaction
    let mut transaction_builder = iota_online.message();
    for input in inputs {
        transaction_builder = transaction_builder.with_input(input);
    }
    let (essence, address_index_recorders) = transaction_builder
        .with_output(address, amount)?
        .prepare_transaction()
        .await?;

    println!("Prepared transaction sending {} to {}", amount, address);

    write_transaction_to_file(
        PREPARED_TRANSACTION_FILE_NAME,
        PreparedTransactionData {
            essence,
            address_index_recorders,
        },
    )?;

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

struct OutputWrapper {
    output: UtxoInput,
    amount: u64,
}

async fn get_inputs(client: &Client, outputs: Vec<UtxoInput>, amount: u64) -> Result<Vec<UtxoInput>> {
    let mut signature_locked_outputs = Vec::new();
    let mut dust_allowance_outputs = Vec::new();

    for output in outputs.into_iter() {
        let output_data = client.get_output(&output).await?;
        let (amount, _, signature_locked) = ClientMessageBuilder::get_output_amount_and_address(&output_data.output)?;
        let output_wrapper = OutputWrapper { output, amount };
        if signature_locked {
            signature_locked_outputs.push(output_wrapper);
        } else {
            dust_allowance_outputs.push(output_wrapper);
        }
    }
    signature_locked_outputs.sort_by(|l, r| r.amount.cmp(&l.amount));
    dust_allowance_outputs.sort_by(|l, r| r.amount.cmp(&l.amount));

    let mut total_already_spent = 0;
    let mut selected_inputs = Vec::new();
    for (_offset, output_wrapper) in signature_locked_outputs
        .into_iter()
        .chain(dust_allowance_outputs.into_iter())
        // Max inputs is 127
        .take(INPUT_OUTPUT_COUNT_MAX)
        .enumerate()
    {
        // Break if we have enough funds and don't create dust for the remainder
        if total_already_spent == amount || total_already_spent >= amount + DUST_THRESHOLD {
            break;
        }
        selected_inputs.push(output_wrapper.output.clone());
        total_already_spent += output_wrapper.amount;
    }
    Ok(selected_inputs)
}
