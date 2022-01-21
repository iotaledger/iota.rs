// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example output --release

use iota_client::{
    bee_message::{
        address::{Address, AliasAddress},
        output::{
            AliasId, AliasOutputBuilder, ExtendedOutputBuilder, FoundryOutputBuilder, NftId, NftOutputBuilder, Output,
            TokenScheme,
        },
    },
    signing::mnemonic::MnemonicSigner,
    utils::request_funds_from_faucet,
    Client, Result,
};
use primitive_types::U256;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will send a transaction

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();
    let signer = MnemonicSigner::new(&env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap())?;

    let address = iota.get_addresses(&signer).with_range(0..1).get_all_raw().await?.public[0];
    request_funds_from_faucet(
        "http://localhost:14265/api/plugins/faucet/enqueue",
        &address.to_bech32("atoi"),
    )
    .await?;

    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Extended(
        ExtendedOutputBuilder::new(address.clone(), 1_000_000).finish()?,
    ));
    // outputs.push(Output::Nft(
    //     NftOutputBuilder::new(address, 1_000_000, NftId::from([0; 20]), vec![1, 2, 3])?.finish()?,
    // ));
    // outputs.push(Output::Alias(
    //     AliasOutputBuilder::new(1_000_000, AliasId::new([0; 20]), address.clone(), address.clone())?.finish()?,
    // ));
    // todo make this valid
    // outputs.push(Output::Foundry(
    //     FoundryOutputBuilder::new(
    //         Address::Alias(AliasAddress::new(AliasId::new([0; 20]))),
    //         1_000_000,
    //         0,
    //         [0; 12],
    //         U256::from(100),
    //         U256::from(100),
    //         TokenScheme::Simple,
    //     )?
    //     .finish()?,
    // ));

    let message = iota
        .message()
        .with_signer(&signer)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Transaction sent: http://localhost:14265/api/v1/messages/{}",
        message.id()
    );
    println!(
        "Message metadata: http://localhost:14265/api/v1/messages/{}/metadata",
        message.id()
    );

    // conflict reasons from https://github.com/gohornet/hornet/blob/4cd911a5aaed017c31a2093fc27bf4d06182ac67/pkg/model/storage/message_metadata.go#L31
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
