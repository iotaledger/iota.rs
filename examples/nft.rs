// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example nft --release

use iota_client::{
    bee_message::{
        address::{Address, AliasAddress},
        output::{
            feature_block::IssuerFeatureBlock,
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            FeatureBlock, NftId, NftOutputBuilder, Output, OutputId,
        },
        payload::Payload,
    },
    signing::mnemonic::MnemonicSigner,
    utils::request_funds_from_faucet,
    Client, Result,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will create an NFT output

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
        "http://localhost:14265/api/plugins/faucet/v1/enqueue",
        &address.to_bech32("atoi"),
    )
    .await?;

    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Nft(
        // address of the owner of the NFT
        NftOutputBuilder::new(1_000_000, NftId::from([0; 20]), vec![1, 2, 3])?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            // address of the minter of the NFT
            // .add_feature_block(FeatureBlock::Issuer(IssuerFeatureBlock::new(address)))
            .finish()?,
    ));

    let message = iota
        .message()
        .with_signer(&signer)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!("Message sent: http://localhost:14265/api/v2/messages/{}", message.id());
    let _ = iota.retry_until_included(&message.id(), None, None).await?;

    let tx_id = match message.payload().unwrap() {
        Payload::Transaction(tx_payload) => tx_payload.id(),
        _ => panic!("No tx payload"),
    };
    let nft_output_id = OutputId::new(tx_id, 1)?;
    // create second transaction with the actual NFT id (BLAKE2b-160 hash of the Output ID that created the NFT)
    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Nft(
        // address of the owner of the NFT
        NftOutputBuilder::new(1_000_000, NftId::from(nft_output_id.hash()), vec![1, 2, 3])?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            // address of the minter of the NFT
            // .add_feature_block(FeatureBlock::Issuer(IssuerFeatureBlock::new(address)))
            .finish()?,
    ));

    let message = iota
        .message()
        .with_signer(&signer)
        .with_input(nft_output_id.into())
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Second message sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );
    let _ = iota.retry_until_included(&message.id(), None, None).await?;

    Ok(())
}
