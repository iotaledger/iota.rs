// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example all --release

use iota_client::{
    bee_message::{
        address::AliasAddress,
        milestone::MilestoneIndex,
        output::{
            feature_block::{IssuerFeatureBlock, MetadataFeatureBlock, SenderFeatureBlock},
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, GovernorAddressUnlockCondition,
                ImmutableAliasAddressUnlockCondition, StateControllerAddressUnlockCondition,
                StorageDepositReturnUnlockCondition, TimelockUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, BasicOutputBuilder, FeatureBlock, FoundryId, FoundryOutputBuilder,
            NativeToken, NftId, NftOutputBuilder, Output, OutputId, SimpleTokenScheme, TokenId, TokenScheme, TokenTag,
        },
        payload::{transaction::TransactionEssence, Payload},
    },
    node_api::indexer_api::query_parameters::QueryParameter,
    request_funds_from_faucet,
    signing::mnemonic::MnemonicSigner,
    Client, Result,
};
use primitive_types::U256;
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will create all output types in a single transaction

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        // .with_local_pow(false)
        .with_default_logger()?
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
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    //////////////////////////////////
    // create new alias and nft output
    //////////////////////////////////
    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Alias(
        AliasOutputBuilder::new(2_000_000, AliasId::from([0; 20]))?
            .with_state_index(0)
            .with_foundry_counter(0)
            .add_feature_block(FeatureBlock::Sender(SenderFeatureBlock::new(address)))
            .add_feature_block(FeatureBlock::Metadata(MetadataFeatureBlock::new(vec![1, 2, 3])?))
            .add_immutable_feature_block(FeatureBlock::Issuer(IssuerFeatureBlock::new(address)))
            .add_unlock_condition(UnlockCondition::StateControllerAddress(
                StateControllerAddressUnlockCondition::new(address),
            ))
            .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
                address,
            )))
            .finish()?,
    ));
    outputs.push(Output::Nft(
        // address of the owner of the NFT
        NftOutputBuilder::new(1_000_000, NftId::from([0; 20]))?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            // address of the minter of the NFT
            // .add_feature_block(FeatureBlock::Issuer(IssuerFeatureBlock::new(address)))
            .finish()?,
    ));

    let message = iota
        .message()
        .with_signer(&signer)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Transaction with new nft and alias output sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );
    let _ = iota.retry_until_included(&message.id(), None, None).await?;

    ///////////////////////////////////////////////
    // create foundry, native tokens and nft output
    ///////////////////////////////////////////////
    let alias_output_id = get_alias_output_id(message.payload().unwrap());
    let alias_id = AliasId::from(alias_output_id);

    let nft_output_id = get_nft_output_id(message.payload().unwrap());
    let nft_id = NftId::from(nft_output_id);

    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Alias(
        AliasOutputBuilder::new(1_000_000, alias_id)?
            .with_state_index(1)
            .with_foundry_counter(1)
            .add_feature_block(FeatureBlock::Sender(SenderFeatureBlock::new(address)))
            .add_feature_block(FeatureBlock::Metadata(MetadataFeatureBlock::new(vec![1, 2, 3])?))
            .add_immutable_feature_block(FeatureBlock::Issuer(IssuerFeatureBlock::new(address)))
            .add_unlock_condition(UnlockCondition::StateControllerAddress(
                StateControllerAddressUnlockCondition::new(address),
            ))
            .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
                address,
            )))
            .finish()?,
    ));

    let token_scheme = TokenScheme::Simple(SimpleTokenScheme::new(U256::from(50), U256::from(0), U256::from(100))?);
    let foundry_id = FoundryId::build(&AliasAddress::from(AliasId::from(alias_output_id)), 1, &token_scheme);
    let token_id = TokenId::build(&foundry_id, &TokenTag::new([0u8; 12]));
    outputs.push(Output::Foundry(
        FoundryOutputBuilder::new(1_000_000, 1, TokenTag::new([0u8; 12]), token_scheme)?
            // Mint native tokens
            .add_native_token(NativeToken::new(token_id, U256::from(50))?)
            .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
                ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)),
            ))
            .finish()?,
    ));
    outputs.push(Output::Nft(
        NftOutputBuilder::new(1_000_000, nft_id)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .finish()?,
    ));

    let message = iota
        .message()
        .with_signer(&signer)
        .with_input(nft_output_id.into())?
        .with_input(alias_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with foundry output, minted tokens and nft sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );
    let _ = iota.retry_until_included(&message.id(), None, None).await?;

    //////////////////////////////////
    // create all outputs
    //////////////////////////////////
    let alias_output_id = get_alias_output_id(message.payload().unwrap());
    let foundry_output_id = get_foundry_output_id(message.payload().unwrap());
    let nft_output_id = get_nft_output_id(message.payload().unwrap());
    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Alias(
        AliasOutputBuilder::new(1_000_000, alias_id)?
            .with_state_index(2)
            .with_foundry_counter(1)
            .add_feature_block(FeatureBlock::Sender(SenderFeatureBlock::new(address)))
            .add_feature_block(FeatureBlock::Metadata(MetadataFeatureBlock::new(vec![1, 2, 3])?))
            .add_immutable_feature_block(FeatureBlock::Issuer(IssuerFeatureBlock::new(address)))
            .add_unlock_condition(UnlockCondition::StateControllerAddress(
                StateControllerAddressUnlockCondition::new(address),
            ))
            .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
                address,
            )))
            .finish()?,
    ));
    outputs.push(Output::Foundry(
        FoundryOutputBuilder::new(
            1_000_000,
            1,
            TokenTag::new([0u8; 12]),
            TokenScheme::Simple(SimpleTokenScheme::new(U256::from(50), U256::from(0), U256::from(100))?),
        )?
        .add_unlock_condition(UnlockCondition::ImmutableAliasAddress(
            ImmutableAliasAddressUnlockCondition::new(AliasAddress::from(alias_id)),
        ))
        .finish()?,
    ));
    outputs.push(Output::Nft(
        NftOutputBuilder::new(1_000_000, nft_id)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .finish()?,
    ));
    // with native token
    outputs.push(Output::Basic(
        BasicOutputBuilder::new(1_000_000)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_native_token(NativeToken::new(token_id, U256::from(50))?)
            .finish()?,
    ));
    // with most simple output
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
    // with storage deposit return
    outputs.push(Output::Basic(
        BasicOutputBuilder::new(234100)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .add_unlock_condition(UnlockCondition::StorageDepositReturn(
                StorageDepositReturnUnlockCondition::new(address, 234000)?,
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

    // get additional input for the new basic output
    let output_ids = iota_client::node_api::indexer_api::routes::output_ids(
        &iota,
        vec![QueryParameter::Address(address.to_bech32("atoi"))],
    )
    .await?;

    let message = iota
        .message()
        .with_signer(&signer)
        .with_input(output_ids[0].into())?
        .with_input(nft_output_id.into())?
        .with_input(alias_output_id.into())?
        .with_input(foundry_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with all outputs sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );
    let _ = iota.retry_until_included(&message.id(), None, None).await?;

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

// helper function to get the output id for the first NFT output
fn get_nft_output_id(payload: &Payload) -> OutputId {
    match payload {
        Payload::Transaction(tx_payload) => {
            let TransactionEssence::Regular(regular) = tx_payload.essence();
            for (index, output) in regular.outputs().iter().enumerate() {
                if let Output::Nft(_nft_output) = output {
                    return OutputId::new(tx_payload.id(), index.try_into().unwrap()).unwrap();
                }
            }
            panic!("No nft output in transaction essence")
        }
        _ => panic!("No tx payload"),
    }
}
