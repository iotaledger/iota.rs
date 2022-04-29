// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example native_tokens --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_message::output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, NativeToken, TokenId,
    },
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    utils::request_funds_from_faucet,
    Client, Result,
};
use primitive_types::U256;

/// In this example we will send basic outputs with native tokens

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();
    let secret_manager = SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(
        &env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap(),
    )?);

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
    request_funds_from_faucet(
        "http://localhost:14265/api/plugins/faucet/v1/enqueue",
        &address.to_bech32("atoi"),
    )
    .await?;

    let token_id: [u8; 38] =
        hex::decode("08e68f7616cd4948efebc6a77c4f93aed770ac53860100000000000000000000000000000000")?
            .try_into()
            .unwrap();
    let outputs = vec![
        // most simple output
        BasicOutputBuilder::new_with_amount(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_native_token(NativeToken::new(TokenId::new(token_id), U256::from(50))?)
            .finish_output()?,
    ];

    let message = client
        .message()
        .with_secret_manager(&secret_manager)
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
