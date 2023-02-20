// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example alias --release

use iota_client::{
    block::{
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature},
            unlock_condition::{
                GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, Feature, Output, OutputId,
        },
        payload::{transaction::TransactionEssence, Payload},
    },
    request_funds_from_faucet,
    secret::SecretManager,
    Client, Result,
};

/// In this example we will create an alias output

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
    request_funds_from_faucet(&faucet_url, &address.to_bech32(client.get_bech32_hrp().await?)).await?;
    tokio::time::sleep(std::time::Duration::from_secs(15)).await;

    //////////////////////////////////
    // create new alias output
    //////////////////////////////////
    let alias_output_builder = AliasOutputBuilder::new_with_amount(1_000_000, AliasId::null())?
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

    //////////////////////////////////
    // create second transaction with the actual AliasId (BLAKE2b-160 hash of the Output ID that created the alias)
    //////////////////////////////////
    let alias_output_id = get_alias_output_id(block.payload().unwrap())?;
    let alias_id = AliasId::from(&alias_output_id);
    let outputs = vec![
        alias_output_builder
            .with_alias_id(alias_id)
            .with_state_index(1)
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
        "Transaction with alias id set sent: {node_url}/api/core/v2/blocks/{}",
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
