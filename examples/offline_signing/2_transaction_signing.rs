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
    api::{PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData, SignedTransactionDataDto},
    bee_message::{payload::transaction::TransactionPayload, unlock_block::UnlockBlocks},
    secret::{mnemonic::MnemonicSecretManager, SecretManageExt, SecretManager},
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

    let prepared_transaction_data = read_prepared_transaction_from_file(PREPARED_TRANSACTION_FILE_NAME)?;

    // Signs prepared transaction offline.
    let unlock_blocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data)
        .await?;
    let unlock_blocks = UnlockBlocks::new(unlock_blocks)?;
    let signed_transaction = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlock_blocks)?;

    let signed_transaction_data = SignedTransactionData {
        transaction_payload: signed_transaction,
        inputs_data: prepared_transaction_data.inputs_data,
    };

    println!("Signed transaction.");

    write_signed_transaction_to_file(SIGNED_TRANSACTION_FILE_NAME, &signed_transaction_data)?;

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

fn write_signed_transaction_to_file<P: AsRef<Path>>(
    path: P,
    signed_transaction_data: &SignedTransactionData,
) -> Result<()> {
    let dto = SignedTransactionDataDto::from(signed_transaction_data);
    let json = serde_json::to_string_pretty(&dto)?;
    let mut file = BufWriter::new(File::create(path)?);

    println!("{}", json);

    file.write_all(json.as_bytes())?;

    Ok(())
}
