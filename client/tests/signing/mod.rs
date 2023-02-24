// Copyright 2023 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod alias;
mod basic;
mod nft;

use std::str::FromStr;

use crypto::keys::slip10::Chain;
use iota_client::{
    api::{
        input_selection::InputSelection, transaction::validate_transaction_payload_length, verify_semantic,
        PreparedTransactionData,
    },
    block::{
        address::{Address, AliasAddress, NftAddress},
        input::{Input, UtxoInput},
        output::{InputsCommitment, NftId},
        payload::{
            transaction::{RegularTransactionEssence, TransactionEssence},
            TransactionPayload,
        },
        protocol::protocol_parameters,
        semantic::ConflictReason,
        unlock::{SignatureUnlock, Unlock},
    },
    constants::{HD_WALLET_TYPE, SHIMMER_COIN_TYPE, SHIMMER_TESTNET_BECH32_HRP},
    secret::{SecretManage, SecretManageExt, SecretManager},
    Result,
};
use iota_types::block::output::AliasId;

use crate::{
    addresses, build_inputs, build_outputs,
    Build::{Alias, Basic, Nft},
    ALIAS_ID_1, ALIAS_ID_2, NFT_ID_1, NFT_ID_2, NFT_ID_3, NFT_ID_4,
};

#[tokio::test]
async fn all_combined() -> Result<()> {
    let secret_manager = SecretManager::try_from_mnemonic(
        // mnemonic needs to be hardcoded to make the ordering deterministic
        "mirror add nothing long orphan hat this rough scare gallery fork twelve old shrug voyage job table obscure mimic holiday possible proud giraffe fan",
    )?;

    let protocol_parameters = protocol_parameters();

    let ed25519_bech32_addresses = secret_manager
        .generate_addresses(SHIMMER_COIN_TYPE, 0, 0..3, false, None)
        .await?;
    let ed25519_bech32_address_0 = &ed25519_bech32_addresses[0].to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let ed25519_bech32_address_1 = &ed25519_bech32_addresses[1].to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let ed25519_bech32_address_2 = &ed25519_bech32_addresses[2].to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let alias_id_1 = AliasId::from_str(ALIAS_ID_1)?;
    let alias_id_2 = AliasId::from_str(ALIAS_ID_2)?;
    let alias_1_bech32_address = &Address::Alias(AliasAddress::new(alias_id_1)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let alias_2_bech32_address = &Address::Alias(AliasAddress::new(alias_id_2)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let nft_id_1 = NftId::from_str(NFT_ID_1)?;
    let nft_id_2 = NftId::from_str(NFT_ID_2)?;
    let nft_id_3 = NftId::from_str(NFT_ID_3)?;
    let nft_id_4 = NftId::from_str(NFT_ID_4)?;
    let nft_1_bech32_address = &Address::Nft(NftAddress::new(nft_id_1)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let nft_2_bech32_address = &Address::Nft(NftAddress::new(nft_id_2)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let nft_3_bech32_address = &Address::Nft(NftAddress::new(nft_id_3)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);
    let nft_4_bech32_address = &Address::Nft(NftAddress::new(nft_id_4)).to_bech32(SHIMMER_TESTNET_BECH32_HRP);

    let inputs = build_inputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            0,
            nft_1_bech32_address,
            nft_1_bech32_address,
            None,
            None,
            None,
            None,
        ),
        Alias(
            1_000_000,
            alias_id_2,
            0,
            ed25519_bech32_address_0,
            ed25519_bech32_address_1,
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
        ),
        Basic(1_000_000, alias_1_bech32_address, None, None, None, None, None, None),
        Basic(1_000_000, alias_2_bech32_address, None, None, None, None, None, None),
        Basic(1_000_000, alias_2_bech32_address, None, None, None, None, None, None),
        Basic(1_000_000, nft_2_bech32_address, None, None, None, None, None, None),
        Basic(1_000_000, nft_2_bech32_address, None, None, None, None, None, None),
        Basic(1_000_000, nft_4_bech32_address, None, None, None, None, None, None),
        Basic(
            1_000_000,
            ed25519_bech32_address_0,
            None,
            None,
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
        ),
        Basic(
            1_000_000,
            ed25519_bech32_address_1,
            None,
            None,
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
        ),
        Basic(
            1_000_000,
            ed25519_bech32_address_2,
            None,
            None,
            None,
            None,
            None,
            Some(Chain::from_u32_hardened(vec![
                HD_WALLET_TYPE,
                SHIMMER_COIN_TYPE,
                0,
                0,
                2,
            ])),
        ),
        Basic(
            1_000_000,
            ed25519_bech32_address_2,
            None,
            None,
            None,
            None,
            None,
            Some(Chain::from_u32_hardened(vec![
                HD_WALLET_TYPE,
                SHIMMER_COIN_TYPE,
                0,
                0,
                2,
            ])),
        ),
        Nft(
            1_000_000,
            nft_id_1,
            ed25519_bech32_address_0,
            None,
            None,
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
        ),
        Nft(
            1_000_000,
            nft_id_2,
            alias_1_bech32_address,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        // Expirations
        Basic(
            2_000_000,
            ed25519_bech32_address_0,
            None,
            None,
            None,
            None,
            Some((alias_1_bech32_address, 50)),
            None,
        ),
        Basic(
            2_000_000,
            ed25519_bech32_address_0,
            None,
            None,
            None,
            None,
            Some((nft_3_bech32_address, 50)),
            None,
        ),
        Basic(
            2_000_000,
            ed25519_bech32_address_0,
            None,
            None,
            None,
            None,
            Some((nft_3_bech32_address, 150)),
            Some(Chain::from_u32_hardened(vec![
                HD_WALLET_TYPE,
                SHIMMER_COIN_TYPE,
                0,
                0,
                0,
            ])),
        ),
        Nft(
            1_000_000,
            nft_id_3,
            alias_1_bech32_address,
            None,
            None,
            None,
            None,
            Some((nft_4_bech32_address, 50)),
            None,
        ),
        Nft(
            1_000_000,
            nft_id_4,
            alias_1_bech32_address,
            None,
            None,
            None,
            None,
            Some((nft_3_bech32_address, 150)),
            None,
        ),
    ]);

    let outputs = build_outputs(vec![
        Alias(
            1_000_000,
            alias_id_1,
            1,
            nft_1_bech32_address,
            nft_1_bech32_address,
            None,
            None,
            None,
            None,
        ),
        Alias(
            1_000_000,
            alias_id_2,
            1,
            ed25519_bech32_address_0,
            ed25519_bech32_address_1,
            None,
            None,
            None,
            None,
        ),
        Basic(10_000_000, ed25519_bech32_address_0, None, None, None, None, None, None),
        Nft(
            1_000_000,
            nft_id_1,
            ed25519_bech32_address_0,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_2,
            ed25519_bech32_address_0,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_3,
            ed25519_bech32_address_0,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        Nft(
            1_000_000,
            nft_id_4,
            ed25519_bech32_address_0,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
    ]);

    let current_time = 100;

    let selected = InputSelection::new(
        inputs.clone(),
        outputs.clone(),
        addresses(vec![
            ed25519_bech32_address_0,
            ed25519_bech32_address_1,
            ed25519_bech32_address_2,
        ]),
        protocol_parameters.clone(),
    )
    .timestamp(current_time)
    .select()
    .unwrap();

    let essence = TransactionEssence::Regular(
        RegularTransactionEssence::builder(
            protocol_parameters.network_id(),
            InputsCommitment::new(selected.inputs.iter().map(|i| &i.output)),
        )
        .with_inputs(
            selected
                .inputs
                .iter()
                .map(|i| Input::Utxo(UtxoInput::from(*i.output_metadata.output_id())))
                .collect(),
        )
        .with_outputs(outputs)
        .finish(&protocol_parameters)?,
    );

    let prepared_transaction_data = PreparedTransactionData {
        essence,
        inputs_data: selected.inputs,
        remainder: None,
    };

    let unlocks = secret_manager
        .sign_transaction_essence(&prepared_transaction_data, Some(current_time))
        .await?;

    assert_eq!(unlocks.len(), 15);
    assert_eq!((*unlocks).get(0).unwrap().kind(), SignatureUnlock::KIND);
    match (*unlocks).get(1).unwrap() {
        Unlock::Reference(a) => {
            assert_eq!(a.index(), 0);
        }
        _ => panic!("Invalid unlock 1"),
    }
    assert_eq!((*unlocks).get(2).unwrap().kind(), SignatureUnlock::KIND);
    assert_eq!((*unlocks).get(3).unwrap().kind(), SignatureUnlock::KIND);
    match (*unlocks).get(4).unwrap() {
        Unlock::Reference(a) => {
            assert_eq!(a.index(), 3);
        }
        _ => panic!("Invalid unlock 4"),
    }
    match (*unlocks).get(5).unwrap() {
        Unlock::Reference(a) => {
            assert_eq!(a.index(), 3);
        }
        _ => panic!("Invalid unlock 5"),
    }
    match (*unlocks).get(6).unwrap() {
        Unlock::Alias(a) => {
            assert_eq!(a.index(), 5);
        }
        _ => panic!("Invalid unlock 6"),
    }
    match (*unlocks).get(7).unwrap() {
        Unlock::Alias(a) => {
            assert_eq!(a.index(), 5);
        }
        _ => panic!("Invalid unlock 7"),
    }
    match (*unlocks).get(8).unwrap() {
        Unlock::Reference(a) => {
            assert_eq!(a.index(), 3);
        }
        _ => panic!("Invalid unlock 8"),
    }

    match (*unlocks).get(9).unwrap() {
        Unlock::Nft(a) => {
            assert_eq!(a.index(), 8);
        }
        _ => panic!("Invalid unlock 9"),
    }
    match (*unlocks).get(10).unwrap() {
        Unlock::Alias(a) => {
            assert_eq!(a.index(), 9);
        }
        _ => panic!("Invalid unlock 10"),
    }
    match (*unlocks).get(11).unwrap() {
        Unlock::Alias(a) => {
            assert_eq!(a.index(), 9);
        }
        _ => panic!("Invalid unlock 11"),
    }
    match (*unlocks).get(12).unwrap() {
        Unlock::Alias(a) => {
            assert_eq!(a.index(), 9);
        }
        _ => panic!("Invalid unlock 12"),
    }
    match (*unlocks).get(13).unwrap() {
        Unlock::Nft(a) => {
            assert_eq!(a.index(), 11);
        }
        _ => panic!("Invalid unlock 13"),
    }
    match (*unlocks).get(14).unwrap() {
        Unlock::Nft(a) => {
            assert_eq!(a.index(), 10);
        }
        _ => panic!("Invalid unlock 14"),
    }

    let tx_payload = TransactionPayload::new(prepared_transaction_data.essence.clone(), unlocks)?;

    validate_transaction_payload_length(&tx_payload)?;

    let conflict = verify_semantic(&prepared_transaction_data.inputs_data, &tx_payload, current_time)?;

    if conflict != ConflictReason::None {
        panic!("{conflict:?}, with {tx_payload:#?}");
    }

    Ok(())
}
