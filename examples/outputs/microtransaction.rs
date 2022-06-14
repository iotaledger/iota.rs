// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example foo --release

use std::{
    env,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use dotenv::dotenv;
use iota_client::{
    bee_block::{
        output::{
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition, UnlockCondition,
            },
            BasicOutputBuilder,
        },
        payload::milestone::MilestoneIndex,
    },
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
};

/// In this example we will send basic outputs with different feature blocks

const NODE_URL: &'static str = "http://localhost:14265";

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance.
    let client = Client::builder()
        .with_node(NODE_URL)?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production!
    // Configure your own mnemonic in the ".env" file. Since the output amount cannot be zero, the seed must contain
    // non-zero balance.
    dotenv().ok();
    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap(),
    )?);

    let addresses = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];

    let tomorrow = (SystemTime::now() + Duration::from_secs(24 * 3600))
        .duration_since(UNIX_EPOCH)
        .expect("clock went backwards")
        .as_secs()
        .try_into()
        .unwrap();
    let outputs = vec![
        // with storage deposit return
        BasicOutputBuilder::new_with_amount(255100)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_unlock_condition(UnlockCondition::StorageDepositReturn(
                StorageDepositReturnUnlockCondition::new(address, 255000)?,
            ))
            .add_unlock_condition(UnlockCondition::Expiration(
                ExpirationUnlockCondition::new(address, MilestoneIndex(0), tomorrow).unwrap(),
            ))
            .finish_output()?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!("Transaction sent: {NODE_URL}/api/v2/blocks/{}", block.id());
    println!("Block metadata: {NODE_URL}/api/v2/blocks/{}/metadata", block.id());
    let _ = client.retry_until_included(&block.id(), None, None).await?;
    Ok(())
}
