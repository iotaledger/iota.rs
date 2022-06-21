// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example basic --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_block::output::{
        feature::MetadataFeature,
        unlock_condition::{
            AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
            TimelockUnlockCondition, UnlockCondition,
        },
        BasicOutputBuilder, Feature,
    },
    constants::SHIMMER_TESTNET_BECH32_HRP,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    utils::request_funds_from_faucet,
    Client, Result,
};

/// In this example we will send basic outputs with different feature blocks

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production!
    // Configure your own mnemonic in the ".env" file. Since the output amount cannot be zero, the seed must contain
    // non-zero balance.
    dotenv().ok();

    let node_url = env::var("NODE_URL").unwrap();
    let faucet_url = env::var("FAUCET_URL").unwrap();

    // Create a client instance.
    let client = Client::builder()
        .with_node(&node_url)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap(),
    )?);

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
    println!(
        "{}",
        request_funds_from_faucet(&faucet_url, &address.to_bech32(SHIMMER_TESTNET_BECH32_HRP)).await?
    );

    let outputs = vec![
        // most simple output
        BasicOutputBuilder::new_with_amount(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .finish_output()?,
        // with metadata feature block
        BasicOutputBuilder::new_with_amount(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_feature(Feature::Metadata(MetadataFeature::new(vec![13, 37])?))
            .finish_output()?,
        // with storage deposit return
        BasicOutputBuilder::new_with_amount(234100)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_unlock_condition(UnlockCondition::StorageDepositReturn(
                StorageDepositReturnUnlockCondition::new(address, 234000)?,
            ))
            .finish_output()?,
        // with expiration
        BasicOutputBuilder::new_with_amount(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_unlock_condition(UnlockCondition::Expiration(ExpirationUnlockCondition::new(address, 1)?))
            .finish_output()?,
        // with timelock
        BasicOutputBuilder::new_with_amount(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_unlock_condition(UnlockCondition::Timelock(TimelockUnlockCondition::new(1)?))
            .finish_output()?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!("Transaction sent: {node_url}/api/core/v2/blocks/{}", block.id());
    println!("Block metadata: {node_url}/api/core/v2/blocks/{}/metadata", block.id());
    let _ = client.retry_until_included(&block.id(), None, None).await?;
    Ok(())
}
