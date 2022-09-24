// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! In this example we sign the prepared transaction.
//! This example uses dotenv, which is not safe for use in production.
//! `cargo run --example 2_transaction_signing --release`.

use std::{
    fs::File,
    io::{prelude::*, BufWriter},
    path::Path,
};

use iota_client::{
    api::{PreparedTransactionData, PreparedTransactionDataDto, SignedTransactionData, SignedTransactionDataDto},
    block::{output::RentStructureBuilder, payload::transaction::TransactionPayload, protocol::ProtocolParameters},
    secret::{mnemonic::MnemonicSecretManager, SecretManageExt, SecretManager},
    Result,
};

const PREPARED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/prepared_transaction.json";
const SIGNED_TRANSACTION_FILE_NAME: &str = "examples/offline_signing/signed_transaction.json";

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
        &std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap(),
    )?);

    let prepared_transaction_data = read_prepared_transaction_from_file(PREPARED_TRANSACTION_FILE_NAME)?;

    // Signs the prepared transaction offline.
    let unlocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data)
        .await?;
    let signed_transaction = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    let signed_transaction_data = SignedTransactionData {
        transaction_payload: signed_transaction,
        inputs_data: prepared_transaction_data.inputs_data,
    };

    println!("Signed transaction.");

    write_signed_transaction_to_file(SIGNED_TRANSACTION_FILE_NAME, &signed_transaction_data)?;

    Ok(())
}

fn read_prepared_transaction_from_file<P: AsRef<Path>>(path: P) -> Result<PreparedTransactionData> {
    let mut file = File::open(&path).unwrap();
    let mut json = String::new();
    file.read_to_string(&mut json).unwrap();

    // TODO: read from file https://github.com/iotaledger/iota.rs/issues/1267
    // Make sure that these values match the network you use.
    let protocol_parameters = ProtocolParameters::new(
        2,
        String::from("testnet"),
        String::from("smr"),
        1500,
        15,
        RentStructureBuilder::new()
            .byte_cost(100)
            .key_factor(1)
            .data_factor(10)
            .finish(),
        1813620509061365,
    )
    .unwrap();

    Ok(PreparedTransactionData::try_from_dto(
        &serde_json::from_str::<PreparedTransactionDataDto>(&json)?,
        &protocol_parameters,
    )?)
}

fn write_signed_transaction_to_file<P: AsRef<Path>>(
    path: P,
    signed_transaction_data: &SignedTransactionData,
) -> Result<()> {
    let dto = SignedTransactionDataDto::from(signed_transaction_data);
    let json = serde_json::to_string_pretty(&dto)?;
    let mut file = BufWriter::new(File::create(path).unwrap());

    println!("{}", json);

    file.write_all(json.as_bytes()).unwrap();

    Ok(())
}
