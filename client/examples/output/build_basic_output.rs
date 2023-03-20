// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example build_basic_output --release

use iota_client::{
    block::{
        address::Address,
        output::{
            feature::MetadataFeature,
            unlock_condition::{
                AddressUnlockCondition, ExpirationUnlockCondition, StorageDepositReturnUnlockCondition,
                TimelockUnlockCondition, UnlockCondition,
            },
            BasicOutputBuilder, Feature,
        },
    },
    Client, Result,
};

/// In this example we will send basic outputs with different feature blocks

#[tokio::main]
async fn main() -> Result<()> {
    // This example uses dotenv, which is not safe for use in production!
    dotenv::dotenv().ok();

    let node_url = std::env::var("NODE_URL").unwrap();

    // Create a client instance.
    let client = Client::builder().with_node(&node_url)?.finish()?;

    let token_supply = client.get_token_supply().await?;

    let address = Address::try_from_bech32("rms1qpllaj0pyveqfkwxmnngz2c488hfdtmfrj3wfkgxtk4gtyrax0jaxzt70zy")?.1;

    let basic_output_builder = BasicOutputBuilder::new_with_amount(1_000_000)?
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)));

    let outputs = vec![
        // most simple output
        basic_output_builder.clone().finish_output(token_supply)?,
        // with metadata feature block
        basic_output_builder
            .clone()
            .add_feature(Feature::Metadata(MetadataFeature::new(
                "Hello, World!".as_bytes().to_owned(),
            )?))
            .finish_output(token_supply)?,
        // with storage deposit return
        basic_output_builder
            .clone()
            .add_unlock_condition(UnlockCondition::StorageDepositReturn(
                StorageDepositReturnUnlockCondition::new(address, 1000000, token_supply)?,
            ))
            .finish_output(token_supply)?,
        // with expiration
        basic_output_builder
            .clone()
            .add_unlock_condition(UnlockCondition::Expiration(ExpirationUnlockCondition::new(address, 1)?))
            .finish_output(token_supply)?,
        // with timelock
        basic_output_builder
            .add_unlock_condition(UnlockCondition::Timelock(TimelockUnlockCondition::new(1)?))
            .finish_output(token_supply)?,
    ];

    println!("{outputs:#?}");

    Ok(())
}
