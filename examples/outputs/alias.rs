// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example alias --release

use iota_client::{
    bee_message::{
        output::{
            feature_block::{IssuerFeatureBlock, MetadataFeatureBlock, SenderFeatureBlock},
            unlock_condition::{
                GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, FeatureBlock, Output, OutputId,
        },
        payload::{transaction::TransactionEssence, Payload},
    },
    request_funds_from_faucet,
    signing::mnemonic::MnemonicSigner,
    Client, Result,
};
extern crate dotenv;
use dotenv::dotenv;
use std::env;

/// In this example we will create an alias output

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();
    let signer = MnemonicSigner::new(&env::var("NONSECURE_USE_OF_DEVELOPMENT_MNEMONIC1").unwrap())?;

    let address = iota.get_addresses(&signer).with_range(0..1).get_raw().await?[0];
    request_funds_from_faucet(
        "http://localhost:14265/api/plugins/faucet/v1/enqueue",
        &address.to_bech32("atoi"),
    )
    .await?;
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    //////////////////////////////////
    // create new alias output
    //////////////////////////////////
    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Alias(
        AliasOutputBuilder::new(1_000_000, AliasId::from([0; 20]))?
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

    let message = iota
        .message()
        .with_signer(&signer)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Transaction with new alias output sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );
    let _ = iota.retry_until_included(&message.id(), None, None).await?;

    //////////////////////////////////
    // create second transaction with the actual AliasId (BLAKE2b-160 hash of the Output ID that created the alias)
    //////////////////////////////////
    let alias_output_id = get_alias_output_id(message.payload().unwrap());
    let alias_id = AliasId::from(&alias_output_id);
    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Alias(
        AliasOutputBuilder::new(1_000_000, alias_id)?
            .with_state_index(1)
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

    let message = iota
        .message()
        .with_signer(&signer)
        .with_input(alias_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Transaction with alias id set sent: http://localhost:14265/api/v2/messages/{}",
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
    };
}
