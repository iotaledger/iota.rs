// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example participation --features=participation --release

use iota_client::{
    api_types::plugins::participation::types::{
        Participation, ParticipationEventId, Participations, PARTICIPATION_TAG,
    },
    block::output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, UnlockCondition},
    node_api::indexer::query_parameters::QueryParameter,
    request_funds_from_faucet,
    secret::SecretManager,
    Client, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    // Command to create an event:
    // curl -X POST http://localhost:14265/api/participation/v1/admin/events -H 'Content-Type: application/json' -d '{"name":"Shimmer Proposal","milestoneIndexCommence":580,"milestoneIndexStart":600,"milestoneIndexEnd":700,"payload":{"type":0,"questions":[{"text":"Should we be on CMC rank #1 eoy?","answers":[{"value":1,"text":"Yes","additionalInfo":""},{"value":2,"text":"No","additionalInfo":""}],"additionalInfo":""}]},"additionalInfo":"Nothing needed here"}'
    // Command to delete an event:
    // curl -X DELETE http://localhost:14265/api/participation/v1/admin/events/0x30bec90738f04b72e44ca853f98d90d19fb1c6b06eebdae3cc744439cbcb7e68

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses dotenv, which is not safe for use in production.
        dotenv::dotenv().ok();
        std::env::var("NODE_URL").unwrap()
    });

    // Create a client with that node.
    let client = Client::builder().with_node(&node_url)?.finish()?;

    // Get the participation events.
    let events = client.events(None).await?;

    println!("{events:#?}");

    for event_id in &events.event_ids {
        let event_info = client.event(event_id).await?;
        println!("{event_info:#?}");
        let event_status = client.event_status(event_id, None).await?;
        println!("{event_status:#?}");
    }

    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;
    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];

    let bech32_address = address.to_bech32(client.get_bech32_hrp().await?);

    let address_participation = client.address_staking_status(&bech32_address).await?;
    println!("{address_participation:#?}");

    let address_output_ids = client.address_participation_output_ids(&bech32_address).await?;
    println!("{address_output_ids:#?}");

    for (output_id, _) in address_output_ids.outputs.into_iter() {
        let output_status = client.output_status(&output_id).await?;
        println!("{output_status:#?}");
    }

    // Get outputs for address and request if they're participating
    let output_ids_response = client
        .basic_output_ids(vec![
            QueryParameter::Address(bech32_address),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    for output_id in output_ids_response.items {
        if let Ok(output_status) = client.output_status(&output_id).await {
            println!("{output_status:#?}");
        }
    }

    // Participate with one answer set to `1` for the first event
    participate(
        &client,
        events.event_ids.first().expect("No event available").to_owned(),
        node_url,
    )
    .await?;
    Ok(())
}

async fn participate(client: &Client, event_id: ParticipationEventId, node_url: String) -> Result<()> {
    let secret_manager =
        SecretManager::try_from_mnemonic(&std::env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let token_supply = client.get_token_supply().await?;
    let rent_structure = client.get_rent_structure().await?;

    let address = client.get_addresses(&secret_manager).with_range(0..1).get_raw().await?[0];

    let faucet_url = std::env::var("FAUCET_URL").unwrap();
    request_funds_from_faucet(&faucet_url, &address.to_bech32(client.get_bech32_hrp().await?)).await?;

    let outputs = vec![
        BasicOutputBuilder::new_with_minimum_storage_deposit(rent_structure)?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .finish_output(token_supply)?,
    ];

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(outputs)?
        .with_tag(PARTICIPATION_TAG.as_bytes().to_vec())
        .with_data(
            Participations {
                participations: vec![Participation {
                    event_id,
                    answers: vec![1],
                }],
            }
            .to_bytes()?,
        )
        .finish()
        .await?;

    println!("{block:#?}");

    println!("Transaction sent: {node_url}/api/core/v2/blocks/{}", block.id());
    println!("Block metadata: {node_url}/api/core/v2/blocks/{}/metadata", block.id());
    Ok(())
}
