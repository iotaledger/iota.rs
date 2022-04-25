// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example send_all --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_message::output::{
        unlock_condition::AddressUnlockCondition, BasicOutputBuilder, NativeTokensBuilder, Output, UnlockCondition,
    },
    node_api::indexer::query_parameters::QueryParameter,
    signing::mnemonic::MnemonicSigner,
    Client, Result,
};

/// In this example we will get the outputs of the first address of the seed and send everything
/// Run the consolidation example first if there are more than 128 outputs

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance
    let client = Client::builder()
        .with_node("http://localhost:14265")? // Insert your node URL here
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();

    let signer = MnemonicSigner::new(&env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let seed_2 = MnemonicSigner::new_from_seed(&env::var("NON_SECURE_USE_OF_DEVELOPMENT_SEED_2").unwrap())?;

    // Get output ids of outputs that can be controlled by this address without further unlock constraints
    let output_ids = client
        .output_ids(vec![
            QueryParameter::Address(client.get_addresses(&signer).with_range(0..1).finish().await?[0].clone()),
            QueryParameter::HasExpirationCondition(false),
            QueryParameter::HasTimelockCondition(false),
            QueryParameter::HasStorageDepositReturnCondition(false),
        ])
        .await?;

    // Get the outputs by their id
    let outputs_responses = client.get_outputs(output_ids).await?;

    // Calculate the total amount and native tokens
    let mut total_amount = 0;
    let mut total_native_tokens = NativeTokensBuilder::new();

    for output_response in outputs_responses.into_iter() {
        let output = Output::try_from(&output_response.output)?;

        if let Some(native_tokens) = output.native_tokens() {
            total_native_tokens.add_native_tokens(native_tokens.clone())?;
            // for native_token in native_tokens.iter() {
            //     match total_native_tokens.entry(*native_token.token_id()) {
            //         Entry::Vacant(e) => {
            //             e.insert(*native_token.amount());
            //         }
            //         Entry::Occupied(mut e) => {
            //             *e.get_mut() += *native_token.amount();
            //         }
            //     }
            // }
        }
        total_amount += output.amount();
    }

    let total_native_tokens = total_native_tokens.finish()?;

    println!("Total amount: {}", total_amount);

    let mut basic_output_builder =
        BasicOutputBuilder::new_with_amount(total_amount)?.add_unlock_condition(UnlockCondition::Address(
            AddressUnlockCondition::new(client.get_addresses(&seed_2).with_range(0..1).get_raw().await?[0]),
        ));

    for native_token in total_native_tokens.into_iter() {
        basic_output_builder = basic_output_builder.add_native_token(native_token);
    }
    let new_output = Output::Basic(basic_output_builder.finish()?);

    let message = client
        .message()
        .with_signer(&signer)
        .with_outputs(vec![new_output])?
        .finish()
        .await?;

    println!(
        "Transaction sent: https://explorer.iota.org/devnet/message/{}",
        message.id()
    );

    let _ = client.retry_until_included(&message.id(), None, None).await.unwrap();
    Ok(())
}
