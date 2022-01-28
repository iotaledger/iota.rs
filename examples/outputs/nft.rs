// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example nft --release

use iota_client::{
    bee_message::{
        address::{Address, NftAddress},
        input::UtxoInput,
        output::{
            feature_block::IssuerFeatureBlock,
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            ExtendedOutputBuilder, FeatureBlock, NftId, NftOutputBuilder, Output, OutputId,
        },
        payload::Payload,
    },
    node_api::indexer_api::query_parameters::QueryParameter,
    request_funds_from_faucet,
    signing::mnemonic::MnemonicSigner,
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
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

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
    //////////////////////////////////
    // create second transaction with the actual NFT id (BLAKE2b-160 hash of the Output ID that created the NFT)
    //////////////////////////////////
    let nft_output_id = OutputId::new(tx_id, 1)?;
    let nft_id = NftId::from(nft_output_id.hash());
    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Nft(
        // address of the owner of the NFT
        NftOutputBuilder::new(1_000_000, nft_id, vec![1, 2, 3])?
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

    //////////////////////////////////
    // move funds from an NFT address
    //////////////////////////////////
    // get new output id
    let tx_id = match message.payload().unwrap() {
        Payload::Transaction(tx_payload) => tx_payload.id(),
        _ => panic!("No tx payload"),
    };
    let nft_output_id = OutputId::new(tx_id, 0)?;

    let nft_address = NftAddress::new(nft_id);
    let bech32_nft_address = Address::Nft(nft_address).to_bech32("atoi");
    println!("bech32_nft_address {bech32_nft_address}");
    println!(
        "Faucet request {:?}",
        request_funds_from_faucet(
            "http://localhost:14265/api/plugins/faucet/v1/enqueue",
            &bech32_nft_address,
        )
        .await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    let output_ids = iota_client::node_api::indexer_api::routes::output_ids(
        &iota,
        vec![QueryParameter::Address(bech32_nft_address)],
    )
    .await?;
    println!("Output id for nft address: {:?}", output_ids);
    let output_response = iota.get_output(&output_ids[0]).await?;
    let output = Output::try_from(&output_response.output)?;

    let message = iota
        .message()
        .with_signer(&signer)
        .with_input(nft_output_id.into())
        .with_input(output_ids[0].into())
        .with_outputs(vec![Output::Nft(
            NftOutputBuilder::new(1_000_000 + output.amount(), nft_id, vec![1, 2, 3])?
                .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
                .finish()?,
        )])?
        .finish()
        .await?;

    println!(
        "Transaction sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );

    let _ = iota.retry_until_included(&message.id(), None, None).await?;
    let tx_id = match message.payload().unwrap() {
        Payload::Transaction(tx_payload) => tx_payload.id(),
        _ => panic!("No tx payload"),
    };
    let nft_output_id = OutputId::new(tx_id, 0)?;
    println!("new output_id: {:?}", nft_output_id);

    //////////////////////////////////
    // burn NFT
    //////////////////////////////////
    let tx_id = match message.payload().unwrap() {
        Payload::Transaction(tx_payload) => tx_payload.id(),
        _ => panic!("No tx payload"),
    };
    let nft_output_id = OutputId::new(tx_id, 0)?;
    let output_response = iota.get_output(&nft_output_id).await?;
    let output = Output::try_from(&output_response.output)?;
    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Extended(
        ExtendedOutputBuilder::new(output.amount())
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
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
