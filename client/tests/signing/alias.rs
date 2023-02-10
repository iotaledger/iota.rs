// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use crypto::keys::slip10::Chain;
use iota_client::{
    api::{transaction::validate_transaction_payload_length, verify_semantic, PreparedTransactionData},
    block::{
        input::{Input, UtxoInput},
        output::InputsCommitment,
        payload::{
            transaction::{RegularTransactionEssence, TransactionEssence},
            TransactionPayload,
        },
        protocol::protocol_parameters,
        semantic::ConflictReason,
    },
    constants::{HD_WALLET_TYPE, SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{mnemonic::MnemonicSecretManager, SecretManage, SecretManageExt, SecretManager},
    Client, Result,
};
use iota_types::block::output::AliasId;

use crate::{build_inputs, build_outputs, Build::Alias, ALIAS_ID_2};

#[tokio::test]
async fn sign_alias_state_transition() -> Result<()> {
    let secret_manager =
        SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(&Client::generate_mnemonic()?)?);

    let bech32_address_0 = &secret_manager
        .generate_addresses(SHIMMER_COIN_TYPE, 0, 0..1, false, None)
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let bech32_address_1 = &secret_manager
        .generate_addresses(SHIMMER_COIN_TYPE, 0, 1..2, false, None)
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2)?;

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        bech32_address_0,
        bech32_address_1,
        None,
        None,
        None,
        Some(Chain::from_u32_hardened(vec![
            HD_WALLET_TYPE,
            SHIMMER_COIN_TYPE,
            0,
            0,
            0,
        ])),
    )]);

    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        1,
        bech32_address_0,
        bech32_address_1,
        None,
        None,
        None,
        None,
    )]);

    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(
            protocol_parameters.network_id(),
            InputsCommitment::new(inputs.iter().map(|i| &i.output)),
        )
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect(),
        )
        .with_outputs(outputs)
        .finish(&protocol_parameters)?,
    );

    let prepared_transaction_data = PreparedTransactionData {
        essence,
        inputs_data: inputs,
        remainder: None,
    };

    let unlocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data)
        .await?;
    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    validate_transaction_payload_length(&tx_payload)?;

    let current_time = 100;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

    if conflict != ConflictReason::None {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}

#[tokio::test]
async fn sign_alias_governance_transition() -> Result<()> {
    let secret_manager =
        SecretManager::Mnemonic(MnemonicSecretManager::try_from_mnemonic(&Client::generate_mnemonic()?)?);

    let bech32_address_0 = &secret_manager
        .generate_addresses(SHIMMER_COIN_TYPE, 0, 0..1, false, None)
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let bech32_address_1 = &secret_manager
        .generate_addresses(SHIMMER_COIN_TYPE, 0, 1..2, false, None)
        .await?[0]
        .to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let protocol_parameters = protocol_parameters();
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2)?;

    let inputs = build_inputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        bech32_address_0,
        bech32_address_1,
        None,
        None,
        None,
        Some(Chain::from_u32_hardened(vec![
            HD_WALLET_TYPE,
            SHIMMER_COIN_TYPE,
            0,
            0,
            1,
        ])),
    )]);

    let outputs = build_outputs(vec![Alias(
        1_000_000,
        alias_id_2,
        0,
        bech32_address_0,
        bech32_address_1,
        None,
        None,
        None,
        None,
    )]);

    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(
            protocol_parameters.network_id(),
            InputsCommitment::new(inputs.iter().map(|i| &i.output)),
        )
        .with_inputs(
            inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect(),
        )
        .with_outputs(outputs)
        .finish(&protocol_parameters)?,
    );

    let prepared_transaction_data = PreparedTransactionData {
        essence,
        inputs_data: inputs,
        remainder: None,
    };

    let unlocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data)
        .await?;
    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    validate_transaction_payload_length(&tx_payload)?;

    let current_time = 100;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

    if conflict != ConflictReason::None {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}
