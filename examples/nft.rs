// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example nft --release

use iota_client::{
    bee_message::{
        address::{Address, AliasAddress},
        output::{feature_block::IssuerFeatureBlock, FeatureBlock, NftId, NftOutputBuilder, Output},
    },
    signing::mnemonic::MnemonicSigner,
    utils::{init_logger, request_funds_from_faucet, LevelFilter},
    Client, Result,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will create an NFT output

#[tokio::main]
async fn main() -> Result<()> {
    init_logger("iota.rs.log", LevelFilter::Debug)?;
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
    outputs.push(Output::Nft(
        // address of the owner of the NFT
        NftOutputBuilder::new(address, 1_000_000, NftId::from([0; 20]), vec![1, 2, 3])?
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
    // todo create second transaction with the actual NFT (BLAKE2b-160 hash of the Output ID that created the NFT)

    Ok(())
}
