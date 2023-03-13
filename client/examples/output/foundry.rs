// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example foundry --release

use iota_client::{
    api::input_selection::Burn,
    block::{
        address::AliasAddress,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature},
            unlock_condition::{
                AddressUnlockCondition, GovernorAddressUnlockCondition, ImmutableAliasAddressUnlockCondition,
                StateControllerAddressUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, BasicOutputBuilder, Feature, FoundryId, FoundryOutputBuilder, NativeToken,
            Output, OutputId, SimpleTokenScheme, TokenId, TokenScheme,
        },
        payload::{transaction::TransactionEssence, Payload},
    },
    node_api::indexer::query_parameters::QueryParameter,
    request_funds_from_faucet,
    secret::SecretManager,
    Client, Result,
};
use primitive_types::U256;

/// In this example we will create a foundry output

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production!
    // Configure your own mnemonic in the ".env" file. Since the output amount cannot be zero, the seed must contain
    // non-zero balance.
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();
    let faucet_url = std::env::var("FAUCET_URL").unwrap();

    // Create a client instance.
    let client = Client::builder().with_node(&node_url)?.finish()?;

    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let token_supply = client.get_token_supply().await?;

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
    println!(
        "{}",
        request_funds_from_faucet(&faucet_url, &address.to_bech32(client.get_bech32_hrp().await?)).await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    //////////////////////////////////
    // create new alias output
    //////////////////////////////////

    let alias_output_builder = AliasOutputBuilder::new_with_amount(2_000_000, AliasId::null())?
        .add_feature(Feature::Sender(SenderFeature::new(address)))
        .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
        .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
        .add_unlock_condition(UnlockCondition::StateControllerAddress(
            StateControllerAddressUnlockCondition::new(address),
        ))
        .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
            address,
        )));

    let outputs = vec![alias_output_builder.clone().finish_output(token_supply)?];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Transaction with new alias output sent: {node_url}/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////////////////////
    // create foundry output and mint 70 native tokens
    //////////////////////////////////////////////////

    let alias_output_id = get_alias_output_id(block.payload().unwrap())?;
    let alias_id = AliasId::from(&alias_output_id);
    let token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(
        U256::from(70u8),
        U256::from(0u8),
        U256::from(100u8),
    )?);
    let foundry_id = FoundryId::build(
        &AliasAddress::from(AliasId::from(&alias_output_id)),
        1,
        token_scheme.kind(),
    );
    let token_id = TokenId::from(foundry_id);
    let outputs = vec![
        alias_output_builder
            .clone()
            .with_amount(1_000_000)?
            .with_alias_id(alias_id)
            .with_state_index(1)
            .with_foundry_counter(1)
            .finish_output(token_supply)?,
        FoundryOutputBuilder::new_with_amount(1_000_000, 1, token_scheme)?
            .add_native_token(NativeToken::new(token_id, U256::from(70u8))?)
            .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
                ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)),
            ))
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(alias_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with foundry output sent: {node_url}/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // melt 20 native token
    //////////////////////////////////

    let foundry_output_builder = FoundryOutputBuilder::new_with_amount(
        1_000_000,
        1,
        TokenScheme::Simple(SimpleTokenScheme::new(
            U256::from(70u8),
            U256::from(20u8),
            U256::from(100u8),
        )?),
    )?
    .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
        ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)),
    ));

    let alias_output_id = get_alias_output_id(block.payload().unwrap())?;
    let foundry_output_id = get_foundry_output_id(block.payload().unwrap())?;
    let outputs = vec![
        alias_output_builder
            .clone()
            .with_amount(1_000_000)?
            .with_alias_id(alias_id)
            .with_state_index(2)
            .with_foundry_counter(1)
            .finish_output(token_supply)?,
        foundry_output_builder
            .clone()
            .add_native_token(NativeToken::new(token_id, U256::from(50u8))?)
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(alias_output_id.into())?
        .with_input(foundry_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with native tokens burnt sent: {node_url}/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // send native token
    //////////////////////////////////

    let basic_output_builder = BasicOutputBuilder::new_with_amount(57_700)?
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)));

    let alias_output_id = get_alias_output_id(block.payload().unwrap())?;
    let foundry_output_id = get_foundry_output_id(block.payload().unwrap())?;
    let outputs = vec![
        alias_output_builder
            .clone()
            .with_amount(57_700)?
            .with_alias_id(alias_id)
            .with_state_index(3)
            .with_foundry_counter(1)
            .finish_output(token_supply)?,
        foundry_output_builder.finish_output(token_supply)?,
        basic_output_builder
            .clone()
            .add_native_token(NativeToken::new(token_id, U256::from(50u8))?)
            .finish_output(token_supply)?,
    ];

    // get additional input for the new basic output
    let output_ids_response = client
        .basic_output_ids(vec![QueryParameter::Address(
            address.to_bech32(client.get_bech32_hrp().await?),
        )])
        .await?;

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(output_ids_response.items[0].into())?
        .with_input(alias_output_id.into())?
        .with_input(foundry_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with native tokens sent: {node_url}/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // send native token without foundry
    //////////////////////////////////

    let basic_output_id = get_basic_output_id_with_native_tokens(block.payload().unwrap())?;
    let outputs = vec![
        basic_output_builder
            .clone()
            .add_native_token(NativeToken::new(token_id, U256::from(50u8))?)
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(basic_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Second transaction with native tokens sent: {node_url}/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // burn native token without foundry
    //////////////////////////////////

    let basic_output_id = get_basic_output_id_with_native_tokens(block.payload().unwrap())?;
    let outputs = vec![
        basic_output_builder
            .add_native_token(NativeToken::new(token_id, U256::from(30u8))?)
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_burn(Burn::new().add_native_token(token_id, 20))
        .with_input(basic_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Third transaction with native tokens burned sent: {node_url}/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    Ok(())
}

// helper function to get the output id for the first alias output
fn get_alias_output_id(payload: &Payload) -> Result<OutputId> {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Alias(_alias_output) = output {
                    return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
                }
            }
            panic!("No alias output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}

// helper function to get the output id for the first foundry output
fn get_foundry_output_id(payload: &Payload) -> Result<OutputId> {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Foundry(_foundry_output) = output {
                    return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
                }
            }
            panic!("No foundry output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}

// helper function to get the output id for the first basic output with native tokens
fn get_basic_output_id_with_native_tokens(payload: &Payload) -> Result<OutputId> {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Basic(basic_output) = output {
                    if !basic_output.native_tokens().is_empty() {
                        return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
                    }
                }
            }
            panic!("No basic output with native tokens in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}
