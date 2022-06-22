// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example foundry --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_block::{
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
    constants::SHIMMER_TESTNET_BECH32_HRP,
    node_api::indexer::query_parameters::QueryParameter,
    request_funds_from_faucet,
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client, Result,
};
use primitive_types::U256;

/// In this example we will create an foundry output

#[tokio::main]
async fn main() -> Result<()> {
    // Create a client instance.
    let client = Client::builder()
        .with_node("http://localhost:14265")?
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

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];
    println!(
        "{}",
        request_funds_from_faucet(
            "http://localhost:8091/api/enqueue",
            &address.to_bech32(SHIMMER_TESTNET_BECH32_HRP),
        )
        .await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    //////////////////////////////////
    // create new alias output
    //////////////////////////////////
    let outputs = vec![AliasOutputBuilder::new_with_amount(2_000_000, AliasId::null())?
        .with_state_index(0)
        .with_foundry_counter(0)
        .add_feature(Feature::Sender(SenderFeature::new(address)))
        .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
        .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
        .add_unlock_condition(UnlockCondition::StateControllerAddress(
            StateControllerAddressUnlockCondition::new(address),
        ))
        .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
            address,
        )))
        .finish_output()?];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Transaction with new alias output sent: http://localhost:14265/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////////////////////
    // create foundry output and mint 70 native tokens
    //////////////////////////////////////////////////
    let alias_output_id = get_alias_output_id(block.payload().unwrap());
    let alias_id = AliasId::from(alias_output_id);
    let token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(U256::from(70), U256::from(0), U256::from(100))?);
    let foundry_id = FoundryId::build(
        &AliasAddress::from(AliasId::from(alias_output_id)),
        1,
        token_scheme.kind(),
    );
    let token_id = TokenId::from(foundry_id);
    let outputs = vec![
        AliasOutputBuilder::new_with_amount(1_000_000, alias_id)?
            .with_state_index(1)
            .with_foundry_counter(1)
            .add_feature(Feature::Sender(SenderFeature::new(address)))
            .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
            .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
            .add_unlock_condition(UnlockCondition::StateControllerAddress(
                StateControllerAddressUnlockCondition::new(address),
            ))
            .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
                address,
            )))
            .finish_output()?,
        FoundryOutputBuilder::new_with_amount(1_000_000, 1, token_scheme)?
            .add_native_token(NativeToken::new(token_id, U256::from(70))?)
            .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
                ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)),
            ))
            .finish_output()?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(alias_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with foundry output sent: http://localhost:14265/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // melt 20 native token
    //////////////////////////////////
    let alias_output_id = get_alias_output_id(block.payload().unwrap());
    let foundry_output_id = get_foundry_output_id(block.payload().unwrap());
    let outputs = vec![
        AliasOutputBuilder::new_with_amount(1_000_000, alias_id)?
            .with_state_index(2)
            .with_foundry_counter(1)
            .add_feature(Feature::Sender(SenderFeature::new(address)))
            .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
            .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
            .add_unlock_condition(UnlockCondition::StateControllerAddress(
                StateControllerAddressUnlockCondition::new(address),
            ))
            .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
                address,
            )))
            .finish_output()?,
        FoundryOutputBuilder::new_with_amount(
            1_000_000,
            1,
            TokenScheme::Simple(SimpleTokenScheme::new(U256::from(70), U256::from(20), U256::from(100))?),
        )?
        .add_native_token(NativeToken::new(token_id, U256::from(50))?)
        .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
            ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)),
        ))
        .finish_output()?,
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
        "Transaction with native tokens burnt sent: http://localhost:14265/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // send native token
    //////////////////////////////////
    let alias_output_id = get_alias_output_id(block.payload().unwrap());
    let foundry_output_id = get_foundry_output_id(block.payload().unwrap());
    let outputs = vec![
        AliasOutputBuilder::new_with_amount(1_000_000, alias_id)?
            .with_state_index(3)
            .with_foundry_counter(1)
            .add_feature(Feature::Sender(SenderFeature::new(address)))
            .add_feature(Feature::Metadata(MetadataFeature::new(vec![1, 2, 3])?))
            .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
            .add_unlock_condition(UnlockCondition::StateControllerAddress(
                StateControllerAddressUnlockCondition::new(address),
            ))
            .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
                address,
            )))
            .finish_output()?,
        FoundryOutputBuilder::new_with_amount(
            1_000_000,
            1,
            TokenScheme::Simple(SimpleTokenScheme::new(U256::from(70), U256::from(20), U256::from(100))?),
        )?
        .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
            ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)),
        ))
        .finish_output()?,
        BasicOutputBuilder::new_with_amount(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_native_token(NativeToken::new(token_id, U256::from(50))?)
            .finish_output()?,
    ];

    // get additional input for the new basic output
    let output_ids = client
        .basic_output_ids(vec![QueryParameter::Address(
            address.to_bech32(SHIMMER_TESTNET_BECH32_HRP),
        )])
        .await?;

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(output_ids[0].into())?
        .with_input(alias_output_id.into())?
        .with_input(foundry_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with native tokens sent: http://localhost:14265/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // send native token without foundry
    //////////////////////////////////
    let basic_output_id = get_basic_output_id_with_native_tokens(block.payload().unwrap());
    let outputs = vec![BasicOutputBuilder::new_with_amount(1_000_000)?
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
        .add_native_token(NativeToken::new(token_id, U256::from(50))?)
        .finish_output()?];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(basic_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Second transaction with native tokens sent: http://localhost:14265/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // burn native token without foundry
    //////////////////////////////////
    let basic_output_id = get_basic_output_id_with_native_tokens(block.payload().unwrap());
    let outputs = vec![BasicOutputBuilder::new_with_amount(1_000_000)?
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
        .add_native_token(NativeToken::new(token_id, U256::from(30))?)
        .finish_output()?];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_burning_allowed(true)
        .with_input(basic_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Third transaction with native tokens burned sent: http://localhost:14265/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    Ok(())
}

// helper function to get the output id for the first alias output
fn get_alias_output_id(payload: &Payload) -> OutputId {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Alias(_alias_output) = output {
                    return OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap();
                }
            }
            panic!("No alias output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}

// helper function to get the output id for the first foundry output
fn get_foundry_output_id(payload: &Payload) -> OutputId {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Foundry(_foundry_output) = output {
                    return OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap();
                }
            }
            panic!("No foundry output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}

// helper function to get the output id for the first basic output with native tokens
fn get_basic_output_id_with_native_tokens(payload: &Payload) -> OutputId {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Basic(basic_output) = output {
                    if !basic_output.native_tokens().is_empty() {
                        return OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap();
                    }
                }
            }
            panic!("No basic output with native tokens in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}
