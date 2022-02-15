// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example basic --release

use iota_client::{
    bee_message::{
        milestone::MilestoneIndex,
        output::{
            feature_block::MetadataFeatureBlock,
            unlock_condition::{
                AddressUnlockCondition, DustDepositReturnUnlockCondition, ExpirationUnlockCondition,
                TimelockUnlockCondition, UnlockCondition,
            },
            BasicOutputBuilder, FeatureBlock, Output,
        },
    },
    signing::mnemonic::MnemonicSigner,
    utils::request_funds_from_faucet,
    Client, Result,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will send basic outputs with different feature blocks

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

    let address = iota.get_addresses(&signer).with_range(0..1).get_raw().await?[0];
    println!(
        "{}",
        request_funds_from_faucet(
            "http://localhost:14265/api/plugins/faucet/v1/enqueue",
            &address.to_bech32("atoi"),
        )
        .await?
    );

    let mut outputs: Vec<Output> = Vec::new();
    // most simple output
    outputs.push(Output::Basic(
        BasicOutputBuilder::new(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .finish()?,
    ));
    // with metadata feature block
    outputs.push(Output::Basic(
        BasicOutputBuilder::new(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_feature_block(FeatureBlock::Metadata(MetadataFeatureBlock::new(vec![13, 37])?))
            .finish()?,
    ));
    // with dust deposit return
    outputs.push(Output::Basic(
        BasicOutputBuilder::new(234100)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_unlock_condition(UnlockCondition::DustDepositReturn(
                DustDepositReturnUnlockCondition::new(address, 234000)?,
            ))
            .finish()?,
    ));
    // with expiration
    outputs.push(Output::Basic(
        BasicOutputBuilder::new(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_unlock_condition(UnlockCondition::Expiration(ExpirationUnlockCondition::new(
                address,
                MilestoneIndex::new(400),
                0,
            )?))
            .finish()?,
    ));
    // with timelock
    outputs.push(Output::Basic(
        BasicOutputBuilder::new(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_unlock_condition(UnlockCondition::Timelock(TimelockUnlockCondition::new(
                MilestoneIndex::new(400),
                0,
            )?))
            .finish()?,
    ));

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
    let _ = iota.retry_until_included(&message.id(), None, None).await?;
    Ok(())
}
