// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example send_all --release
//! In this example we will get the outputs of the first address of the seed and send everything.
//! Run the consolidation example first if there are more than 128 outputs.

use iota_client::{
    block::output::{
        unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeTokensBuilder, Output, UnlockCondition,
    },
    node_api::indexer::query_parameters::QueryParameter,
    secret::SecretManager,
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production
    // Configure your own mnemonic in ".env". Since the output amount cannot be zero, the mnemonic must contain non-zero
    // balance
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance
    let client = Client::builder()
        .with_node(&node_url)? // Insert your node URL here
        .finish()?;

    let secret_manager_1 =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let secret_manager_2 =
        SecretManager::try_from_hex_seed(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap())?;

    let token_supply = client.get_token_supply().await?;

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids_response = client
        .basic_output_ids(vec![
            QueryParameter::Address(
                client
                    .get_addresses(&secret_manager_1)
                    .with_range(0..1)
                    .finish()
                    .await?[0]
                    .clone(),
            ),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    // Get the outputs by their id
    let outputs_responses = client.get_outputs(output_ids_response.items).await?;

    // Calculate the total amount and native tokens
    let mut total_amount = 0;
    let mut total_native_tokens = NativeTokensBuilder::new();

    for output_response in outputs_responses {
        let output = Output::try_from_dto(&output_response.output, token_supply)?;

        if let Some(native_tokens) = output.native_tokens() {
            total_native_tokens.add_native_tokens(native_tokens.clone())?;
        }
        total_amount += output.amount();
    }

    let total_native_tokens = total_native_tokens.finish()?;

    println!("Total amount: {total_amount}");

    let mut basic_output_builder = BasicOutputBuilder::new_with_amount(total_amount)?.add_unlock_condition(
        UnlockCondition::Address(AddressUnlockCondition::new(
            client
                .get_addresses(&secret_manager_2)
                .with_range(0..1)
                .get_raw()
                .await?[0],
        )),
    );

    for native_token in total_native_tokens {
        basic_output_builder = basic_output_builder.add_native_token(native_token);
    }
    let new_output = basic_output_builder.finish_output(token_supply)?;

    let block = client
        .block()
        .with_secret_manager(&secret_manager_1)
        .with_outputs(vec![new_output])?
        .finish()
        .await?;

    println!(
        "Transaction sent: {}/block/{}",
        std::env::var("EXPLORER_URL").unwrap(),
        block.id()
    );

    let _ = client.retry_until_included(&block.id(), None, None).await?;

    Ok(())
}
