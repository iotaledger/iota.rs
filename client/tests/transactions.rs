// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// These are E2E test samples, so they are ignored by default.

mod common;

use iota_client::{
    block::{
        output::{unlock_condition::AddressUnlockCondition, BasicOutputBuilder, UnlockCondition},
        payload::transaction::TransactionEssence,
    },
    node_api::indexer::query_parameters::QueryParameter,
    Result,
};
use iota_types::block::{output::OutputId, payload::Payload};

use self::common::create_client_and_secret_manager_with_funds;

#[ignore]
#[tokio::test]
async fn send_basic_output() -> Result<()> {
    let (client, secret_manager) = create_client_and_secret_manager_with_funds(None).await?;

    let token_supply = client.get_token_supply().await?;

    let second_address = client.get_addresses(&secret_manager).with_range(1..2).get_raw().await?[0];

    let output = BasicOutputBuilder::new_with_amount(1_000_000)?
        .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(second_address)))
        .finish_output(token_supply)?;

    let block = client
        .block()
        .with_secret_manager(&secret_manager)
        .with_outputs(vec![output.clone()])?
        .finish()
        .await?;

    let output_id = if let Payload::Transaction(tx_payload) = block.payload().unwrap() {
        let TransactionEssence::Regular(essence) = tx_payload.essence();
        // only one input from the faucet
        assert_eq!(essence.inputs().len(), 1);
        // provided output + remainder output
        assert_eq!(essence.outputs().len(), 2);
        // first output == provided output
        assert_eq!(essence.outputs()[0], output);

        OutputId::new(tx_payload.id(), 0)?
    } else {
        panic!("missing transaction payload")
    };

    client.retry_until_included(&block.id(), None, None).await?;

    let bech32_hrp = client.get_bech32_hrp().await?;

    // output can be fetched from the second address
    let output_ids_response = client
        .basic_output_ids(vec![
            QueryParameter::Address(second_address.to_bech32(bech32_hrp)),
            QueryParameter::HasExpiration(false),
            QueryParameter::HasTimelock(false),
            QueryParameter::HasStorageDepositReturn(false),
        ])
        .await?;

    assert_eq!(output_ids_response.items, vec![output_id]);

    Ok(())
}
