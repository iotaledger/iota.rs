// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sign the prepared transaction.
//! This example uses dotenv, which is not safe for use in production.
//! `cargo run --example 2_transaction_signing --release`.

use std::{
    env,
    fs::File,
    io::{prelude::*, BufWriter},
    path::Path,
};

use dotenv::dotenv;
use iota_client::{
    api::{PreparedTransactionData, PreparedTransactionDataDto},
    bee_message::{
        address::Address,
        payload::{transaction::TransactionPayload, Payload},
        unlock_block::UnlockBlocks,
    },
    secret::{mnemonic::MnemonicSecretManager, Network, SecretManageExt, SecretManager, SignMessageMetadata},
    Result,
};

const PREPARED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/prepared_transaction.json";
const SIGNED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap(),
    )?);

    let prepared_transaction = read_prepared_transaction_from_file(PREPARED_TRANSACTION_FILE_NAME)?;

    let mut input_addresses = Vec::new();
    for input_signing_data in &prepared_transaction.input_signing_data_entries {
        let (_bech32_hrp, address) = Address::try_from_bech32(&input_signing_data.bech32_address)?;
        input_addresses.push(address);
    }

    // Signs prepared transaction offline.
    let unlock_blocks = secret_manager
        .sign_transaction_essence(
            &prepared_transaction.essence,
            &prepared_transaction.input_signing_data_entries,
            // TODO set correct data
            SignMessageMetadata {
                remainder_value: 0,
                remainder_deposit_address: None,
                network: Network::Testnet,
            },
        )
        .await?;
    let unlock_blocks = UnlockBlocks::new(unlock_blocks)?;
    let signed_transaction = TransactionPayload::new(prepared_transaction.essence.clone(), unlock_blocks.clone())?;

    println!("Signed transaction.");

    write_signed_transaction_to_file(
        SIGNED_TRANSACTION_FILE_NAME,
        Payload::Transaction(Box::new(signed_transaction)),
    )?;

    Ok(())
}

fn read_prepared_transaction_from_file<P: AsRef<Path>>(path: P) -> Result<PreparedTransactionData> {
    let mut file = File::open(&path)?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;

    Ok(PreparedTransactionData::try_from(&serde_json::from_str::<
        PreparedTransactionDataDto,
    >(&json)?)?)
}

fn write_signed_transaction_to_file<P: AsRef<Path>>(path: P, signed_transaction: Payload) -> Result<()> {
    let json = serde_json::to_string_pretty(&signed_transaction)?;
    let mut file = BufWriter::new(File::create(path)?);

    println!("{}", json);

    file.write_all(json.as_bytes())?;

    Ok(())
}
