// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example native_tokens --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_block::output::{
        unlock_condition::{AddressUnlockCondition, UnlockCondition},
        BasicOutputBuilder, NativeToken, TokenId,
    },
    constants::SHIMMER_TESTNET_BECH32_HRP,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    utils::request_funds_from_faucet,
    Client, Result,
};
use primitive_types::U256;

/// In this example we will send basic outputs with native tokens

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
    request_funds_from_faucet(&faucet_url, &address.to_bech32(SHIMMER_TESTNET_BECH32_HRP)).await?;

    // Replace with the token ID of native tokens you own.
    let token_id: [u8; 38] =
        hex::decode("08e68f7616cd4948efebc6a77c4f935eaed770ac53869cba56d104f2b472a8836d0100000000")?
            .try_into()
            .unwrap();
    let outputs = vec![
        // most simple output
        BasicOutputBuilder::new_with_amount(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_native_token(NativeToken::new(TokenId::new(token_id), U256::from(50))?)
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

    Ok(())
}
