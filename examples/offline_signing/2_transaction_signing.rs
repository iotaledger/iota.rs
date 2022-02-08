// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 2_transaction_signing --release
use iota_client::{
    api::PreparedTransactionData,
    bee_message::{
        address::Address,
        output::Output,
        payload::{transaction::TransactionPayloadBuilder, Payload},
        unlock_block::UnlockBlocks,
    },
    signing::{
        mnemonic::{MnemonicSigner, IOTA_COIN_TYPE},
        verify_unlock_blocks, InputSigningData, Network, SignMessageMetadata,
    },
    Result,
};
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
    // This example uses dotenv, which is not safe for use in production
    dotenv().ok();
    let signer = MnemonicSigner::new(&env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap())?;

    let prepared_transaction_data = read_prepared_transactiondata_from_file(PREPARED_TRANSACTION_FILE_NAME)?;

    let mut tx_inputs = Vec::new();
    let mut input_addresses = Vec::new();
    for input_signing_data in prepared_transaction_data.input_signing_data_entrys {
        let address = Address::try_from_bech32(&input_signing_data.bech32_address)?;
        tx_inputs.push(InputSigningData {
            input: input_signing_data.input,
            address_index: input_signing_data.address_index,
            address_internal: input_signing_data.internal,
            output_kind: Output::try_from(&input_signing_data.output.output)?.kind(),
            address,
            alias_or_nft_address: None,
        });
        input_addresses.push(address);
    }

    // Sign prepared transaction offline
    let mut signer = signer.lock().await;
    let unlock_blocks = signer
        .sign_transaction_essence(
            IOTA_COIN_TYPE,
            0,
            &prepared_transaction_data.essence,
            &mut tx_inputs,
            // todo set correct data
            SignMessageMetadata {
                remainder_value: 0,
                remainder_deposit_address: None,
                network: Network::Testnet,
            },
        )
        .await?;
    let unlock_blocks = UnlockBlocks::new(unlock_blocks)?;
    let signed_transaction = TransactionPayloadBuilder::new()
        .with_essence(prepared_transaction_data.essence)
        .with_unlock_blocks(unlock_blocks)
        .finish()?;

    verify_unlock_blocks(&signed_transaction, input_addresses)?;

    println!("Signed transaction");

    write_signed_transaction_to_file(
        SIGNED_TRANSACTION_FILE_NAME,
        Payload::Transaction(Box::new(signed_transaction)),
    )?;
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
