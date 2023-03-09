// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example build_alias_output --release

use iota_client::{
    block::{
        address::Address,
        output::{
            feature::{IssuerFeature, MetadataFeature, SenderFeature},
            unlock_condition::{
                GovernorAddressUnlockCondition, StateControllerAddressUnlockCondition, UnlockCondition,
            },
            AliasId, AliasOutputBuilder, Feature,
        },
    },
    Client, Result,
};

/// In this example we will build an alias output
#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production!
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance.
    let client = Client::builder().with_node(&node_url)?.finish()?;

    let token_supply = client.get_token_supply().await?;
    let rent_structure = client.get_rent_structure().await?;

    let address = Address::try_from_bech32("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")?.1;

    // Alias id needs to be null the first time
    let alias_output = AliasOutputBuilder::new_with_minimum_storage_deposit(rent_structure, AliasId::null())?
        // `hello` in bytes
        .with_state_metadata(vec![104, 101, 108, 108, 111])
        .add_feature(Feature::Sender(SenderFeature::new(address)))
        .add_feature(Feature::Metadata(MetadataFeature::new(vec![104, 101, 108, 108, 111])?))
        .add_immutable_feature(Feature::Issuer(IssuerFeature::new(address)))
        .add_immutable_feature(Feature::Metadata(MetadataFeature::new(vec![104, 101, 108, 108, 111])?))
        .add_unlock_condition(UnlockCondition::StateControllerAddress(
            StateControllerAddressUnlockCondition::new(address),
        ))
        .add_unlock_condition(UnlockCondition::GovernorAddress(GovernorAddressUnlockCondition::new(
            address,
        )))
        .finish_output(token_supply)?;

    println!("{alias_output:#?}");

    Ok(())
}
