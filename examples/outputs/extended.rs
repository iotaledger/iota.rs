// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example extended --release

use iota_client::{
    bee_message::{
        address::{Address, AliasAddress},
        milestone::MilestoneIndex,
        output::{
            feature_block::MetadataFeatureBlock,
            unlock_condition::{
                AddressUnlockCondition, DustDepositReturnUnlockCondition, ExpirationUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, ExtendedOutputBuilder, FeatureBlock, FoundryOutputBuilder, NftId,
            NftOutputBuilder, Output, TokenScheme,
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

/// In this example we will send extended outputs with different feature blocks

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
    // most simple output
    // outputs.push(Output::Extended(
    //     ExtendedOutputBuilder::new(1_000_000)
    //         .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
    //         .finish()?,
    // ));
    // // with metadata feature block
    // outputs.push(Output::Extended(
    //     ExtendedOutputBuilder::new(1_000_000)
    //         .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
    //         .add_feature_block(FeatureBlock::Metadata(MetadataFeatureBlock::new(vec![13, 37])?))
    //         .finish()?,
    // ));
    // with dust deposit return
    outputs.push(Output::Extended(
        ExtendedOutputBuilder::new(2_000_000)
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_unlock_condition(UnlockCondition::DustDepositReturn(
                DustDepositReturnUnlockCondition::new(address, 1_000_000)?,
            ))
            .finish()?,
    ));
    // with dust expiration
    // outputs.push(Output::Extended(
    //     ExtendedOutputBuilder::new(1_000_000)
    //         .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
    //         .add_unlock_condition(UnlockCondition::Expiration(ExpirationUnlockCondition::new(
    //             address,
    //             MilestoneIndex::new(400),
    //             0,
    //         )))
    //         .finish()?,
    // ));

    let message = iota
        .message()
        .with_signer(&signer)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Transaction sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );
    println!(
        "Message metadata: http://localhost:14265/api/v2/messages/{}/metadata",
        message.id()
    );

    Ok(())
}
