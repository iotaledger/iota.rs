// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example nft --release

use std::env;

use dotenv::dotenv;
use iota_client::{
    bee_message::{
        address::{Address, NftAddress},
        output::{
            unlock_condition::{AddressUnlockCondition, UnlockCondition},
            BasicOutputBuilder, NftId, NftOutputBuilder, Output, OutputId,
        },
        payload::{transaction::TransactionEssence, Payload},
    },
    node_api::indexer::query_parameters::QueryParameter,
    request_funds_from_faucet,
    signing::mnemonic::MnemonicSigner,
    Client, Result,
};

/// In this example we will create an NFT output

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::builder()
        .with_node("http://localhost:14265")?
        .with_node_sync_disabled()
        .finish()
        .await?;

    // This example uses dotenv, which is not safe for use in production
    // Configure your own seed in ".env". Since the output amount cannot be zero, the seed must contain non-zero balance
    dotenv().ok();
    let signer = MnemonicSigner::new(&env::var("NON_SECURE_USE_OF_DEVELOPMENT_MNEMONIC_1").unwrap())?;

    let address = client.get_addresses(&signer).with_range(0..1).get_raw().await?[0];
    request_funds_from_faucet(
        "http://localhost:14265/api/plugins/faucet/v1/enqueue",
        &address.to_bech32("atoi"),
    )
    .await?;
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    //////////////////////////////////
    // create new nft output
    //////////////////////////////////
    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Nft(
        // address of the owner of the NFT
        NftOutputBuilder::new_with_amount(1_000_000, NftId::from([0; 20]))?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            // address of the minter of the NFT
            // .add_feature_block(FeatureBlock::Issuer(IssuerFeatureBlock::new(address)))
            .finish()?,
    ));

    let message = client
        .message()
        .with_signer(&signer)
        .with_outputs(outputs)?
        .finish()
        .await?;

    println!(
        "Transaction with new NFT output sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );
    let _ = client.retry_until_included(&message.id(), None, None).await?;

    //////////////////////////////////
    // move funds from an NFT address
    //////////////////////////////////
    let nft_output_id = get_nft_output_id(message.payload().unwrap());
    let nft_id = NftId::from(nft_output_id);

    let nft_address = NftAddress::new(nft_id);
    let bech32_nft_address = Address::Nft(nft_address).to_bech32("atoi");
    println!("bech32_nft_address {bech32_nft_address}");
    println!(
        "Faucet request {:?}",
        request_funds_from_faucet(
            "http://localhost:14265/api/plugins/faucet/v1/enqueue",
            &bech32_nft_address,
        )
        .await?
    );
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    let output_ids = client
        .output_ids(vec![QueryParameter::Address(bech32_nft_address)])
        .await?;
    let output_response = client.get_output(&output_ids[0]).await?;
    let output = Output::try_from(&output_response.output)?;

    let message = client
        .message()
        .with_signer(&signer)
        .with_input(nft_output_id.into())?
        .with_input(output_ids[0].into())?
        .with_outputs(vec![Output::Nft(
            NftOutputBuilder::new_with_amount(1_000_000 + output.amount(), nft_id)?
                .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
                .finish()?,
        )])?
        .finish()
        .await?;

    println!(
        "Transaction with input(basic output) to NFT output sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );

    let _ = client.retry_until_included(&message.id(), None, None).await?;

    //////////////////////////////////
    // burn NFT
    //////////////////////////////////
    let nft_output_id = get_nft_output_id(message.payload().unwrap());
    let output_response = client.get_output(&nft_output_id).await?;
    let output = Output::try_from(&output_response.output)?;
    let mut outputs: Vec<Output> = Vec::new();
    outputs.push(Output::Basic(
        BasicOutputBuilder::new_with_amount(output.amount())?
            .add_unlock_condition(UnlockCondition::Address(AddressUnlockCondition::new(address)))
            .finish()?,
    ));

    let message = client
        .message()
        .with_signer(&signer)
        .with_input(nft_output_id.into())?
        .with_outputs(outputs)?
        .finish()
        .await?;
    println!(
        "Burn transaction sent: http://localhost:14265/api/v2/messages/{}",
        message.id()
    );
    let _ = client.retry_until_included(&message.id(), None, None).await?;
    Ok(())
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
