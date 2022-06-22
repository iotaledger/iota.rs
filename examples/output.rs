// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example output --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_block::output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder,
    },
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    utils::request_funds_from_faucet,
    Client, Result,
};

/// In this example we will send a transaction

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic must contain non-zero
    // balance
    dotenv().ok();
    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap(),
    )?);

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
    request_funds_from_faucet("http://localhost:8091/api/enqueue", &address.to_bech32("atoi")).await?;

    let outputs = vec![BasicOutputBuilder::new_with_amount(1_000_000)?
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
        .finish_output()?];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Transaction sent: http://localhost:14265/api/core/v2/blocks/{}",
        block.id()
    );
    println!(
        "Block metadata: http://localhost:14265/api/core/v2/blocks/{}/metadata",
        block.id()
    );

    // conflict reasons from https://github.com/gohornet/hornet/blob/4cd911a5aaed017c31a2093fc27bf4d06182ac67/pkg/model/storage/block_metadata.go#L31
    // 	// ConflictInputUTXOAlreadySpent the referenced UTXO was already spent.
    // ConflictInputUTXOAlreadySpent = 1

    // // ConflictInputUTXOAlreadySpentInThisMilestone the referenced UTXO was already spent while confirming this
    // milestone ConflictInputUTXOAlreadySpentInThisMilestone = 2

    // // ConflictInputUTXONotFound the referenced UTXO cannot be found.
    // ConflictInputUTXONotFound = 3

    // // ConflictInputOutputSumMismatch the sum of the inputs and output values does not match.
    // ConflictInputOutputSumMismatch = 4

    // // ConflictInvalidSignature the unlock block signature is invalid.
    // ConflictInvalidSignature = 5

    // // ConflictSemanticValidationFailed the semantic validation failed.
    // ConflictSemanticValidationFailed = 255
    Ok(())
}
