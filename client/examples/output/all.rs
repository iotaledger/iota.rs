// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example all --release

use iota_client::{
    block::{
        address::AliasAddress,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature},
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, GovernorAddressUnlockCondition,
                ImmutableAliasAddressUnlockCondition, StateControllerAddressUnlockCondition,
                StorageDepositReturnUnlockCondition, TimelockUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, BasicOutputBuilder, Feature, FoundryId, FoundryOutputBuilder, NativeToken,
            NftId, NftOutputBuilder, Output, OutputId, SimpleTokenScheme, TokenId, TokenScheme,
        },
        payload::{transaction::TransactionEssence, Payload},
    },
    node_api::indexer::query_parameters::QueryParameter,
    request_funds_from_faucet,
    secret::SecretManager,
    Client, Result,
};
use primitive_types::U256;

/// In this example we will create all output types in a single transaction

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
        request_funds_from_faucet(&faucet_url, &address.to_bech32(client.get_bech32_hrp().await?),).await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    //////////////////////////////////
    // create new alias and nft output
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
    // address of the owner of the NFT
    let nft_output_builder = NftOutputBuilder::new_with_amount(1_000_000, NftId::null())?
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)));
    let outputs = vec![
        alias_output_builder
            .clone()
            .with_state_index(0)
            .with_foundry_counter(0)
            .finish_output(token_supply)?,
        nft_output_builder
            .clone()
            // address of the minter of the NFT
            // .add_feature(Feature::Issuer(IssuerFeature::new(address)))
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Transaction with new nft and alias output sent: {node_url}/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    ///////////////////////////////////////////////
    // create foundry, native tokens and nft output
    ///////////////////////////////////////////////
    let alias_output_id = get_alias_output_id(block.payload().unwrap())?;
    let alias_id = AliasId::from(&alias_output_id);

    let nft_output_id = get_nft_output_id(block.payload().unwrap())?;
    let nft_id = NftId::from(&nft_output_id);

    let token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(U256::from(50), U256::from(0), U256::from(100))?);
    let foundry_id = FoundryId::build(
        &AliasAddress::from(AliasId::from(&alias_output_id)),
        1,
        token_scheme.kind(),
    );
    let token_id = TokenId::from(foundry_id);

    let foundry_output_builder = FoundryOutputBuilder::new_with_amount(1_000_000, 1, token_scheme)?
        .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
            ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)),
        ));

    let outputs = vec![
        alias_output_builder
            .clone()
            .with_amount(1_000_000)?
            .with_alias_id(alias_id)
            .with_state_index(1)
            .with_foundry_counter(1)
            .finish_output(token_supply)?,
        foundry_output_builder
            .clone()
            // Mint native tokens
            .add_native_token(NativeToken::new(token_id, U256::from(50))?)
            .finish_output(token_supply)?,
        nft_output_builder
            .clone()
            .with_nft_id(nft_id)
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(nft_output_id.into())?
        .with_input(alias_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with foundry output, minted tokens and nft sent: {node_url}/api/core/v2/blocks/{}",
        block.id()
    );
    let _ = client.retry_until_included(&block.id(), None, None).await?;

    //////////////////////////////////
    // create all outputs
    //////////////////////////////////
    let alias_output_id = get_alias_output_id(block.payload().unwrap())?;
    let foundry_output_id = get_foundry_output_id(block.payload().unwrap())?;
    let nft_output_id = get_nft_output_id(block.payload().unwrap())?;

    let basic_output_builder = BasicOutputBuilder::new_with_amount(1_000_000)?
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)));

    let outputs = vec![
        alias_output_builder
            .with_amount(1_000_000)?
            .with_alias_id(alias_id)
            .with_state_index(2)
            .with_foundry_counter(1)
            .finish_output(token_supply)?,
        foundry_output_builder.finish_output(token_supply)?,
        nft_output_builder.with_nft_id(nft_id).finish_output(token_supply)?,
        // with native token
        basic_output_builder
            .clone()
            .add_native_token(NativeToken::new(token_id, U256::from(50))?)
            .finish_output(token_supply)?,
        // with most simple output
        basic_output_builder.clone().finish_output(token_supply)?,
        // with metadata feature block
        basic_output_builder
            .clone()
            .add_feature(Feature::Metadata(MetadataFeature::new(vec![13, 37])?))
            .finish_output(token_supply)?,
        // with storage deposit return
        basic_output_builder
            .clone()
            .with_amount(234_100)?
            .add_unlock_condition(UnlockCondition::StorageDepositReturn(
                StorageDepositReturnUnlockCondition::new(address, 234_000, token_supply)?,
            ))
            .finish_output(token_supply)?,
        // with expiration
        basic_output_builder
            .clone()
            .add_unlock_condition(UnlockCondition::Expiration(ExpirationUnlockCondition::new(address, 1)?))
            .finish_output(token_supply)?,
        // with timelock
        basic_output_builder
            .clone()
            .add_unlock_condition(UnlockCondition::Timelock(TimelockUnlockCondition::new(1)?))
            .finish_output(token_supply)?,
    ];

    // get additional input for the new basic output without extra unlock conditions
    let output_ids_response = client
        .basic_output_ids(vec![
            QueryParameter::Address(address.to_bech32(client.get_bech32_hrp().await?)),
            QueryParameter::HasStorageDepositReturn(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasExpiration(false),
        ])
        .await?;

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_input(output_ids_response.items[0].into())?
        .with_input(nft_output_id.into())?
        .with_input(alias_output_id.into())?
        .with_input(foundry_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with all outputs sent: {node_url}/api/core/v2/blocks/{}",
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

// helper function to get the output id for the first NFT output
fn get_nft_output_id(payload: &Payload) -> Result<OutputId> {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Nft(_nft_output) = output {
                    return Ok(OutputId::new(tx_payload.id(), index.try_into().unwrap())?);
                }
            }
            panic!("No nft output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}
