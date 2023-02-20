// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[cfg(feature = "message_interface")]
use iota_client::api::GetAddressesBuilderOptions;
#[cfg(feature = "message_interface")]
use iota_client::message_interface;
#[cfg(feature = "message_interface")]
use iota_client::message_interface::{Message, Response};
#[cfg(feature = "stronghold")]
use iota_client::secret::stronghold::StrongholdSecretManager;
#[cfg(all(feature = "message_interface", feature = "stronghold"))]
use iota_client::secret::types::StrongholdDto;
#[cfg(feature = "message_interface")]
use iota_client::secret::SecretManagerDto;
use iota_client::{
    api::GetAddressesBuilder,
    constants::{IOTA_BECH32_HRP, IOTA_COIN_TYPE, IOTA_TESTNET_BECH32_HRP, SHIMMER_BECH32_HRP, SHIMMER_COIN_TYPE},
    secret::{mnemonic::MnemonicSecretManager, SecretManager},
    Client,
};
use iota_types::block::address::Address;
use serde::{Deserialize, Serialize};

#[tokio::test]
async fn addresses() {
    let secret_manager = SecretManager::Mnemonic(
        MnemonicSecretManager::try_from_hex_seed("0x256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2")
            .unwrap(),
    );

    let addresses = GetAddressesBuilder::new(&secret_manager)
        .with_coin_type(IOTA_COIN_TYPE)
        .with_bech32_hrp(IOTA_TESTNET_BECH32_HRP)
        .with_account_index(0)
        .with_range(0..1)
        .get_all()
        .await
        .unwrap();

    assert_eq!(
        *addresses.public[0],
        "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string()
    );
    assert_eq!(
        *addresses.internal[0],
        "atoi1qprxpfvaz2peggq6f8k9cj8zfsxuw69e4nszjyv5kuf8yt70t2847shpjak".to_string()
    );
}

#[tokio::test]
async fn public_key_to_address() {
    let client = Client::builder().finish().unwrap();
    let hex_public_key = "0x2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a";

    let public_key_address = client
        .hex_public_key_to_bech32_address(hex_public_key, Some("atoi"))
        .await
        .unwrap();

    assert_eq!(
        public_key_address,
        "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string()
    );
}

#[tokio::test]
async fn mnemonic_address_generation_iota() {
    let mnemonic = "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast";
    let secret_manager = SecretManager::try_from_mnemonic(mnemonic).unwrap();

    // account 0, address 0 and 1
    let addresses = GetAddressesBuilder::new(&secret_manager)
        .with_coin_type(IOTA_COIN_TYPE)
        .with_bech32_hrp(IOTA_BECH32_HRP)
        .with_account_index(0)
        .with_range(0..2)
        .finish()
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "iota1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4skprwjg".to_string()
    );
    assert_eq!(
        addresses[1],
        "iota1qpswqe4v8z2cdtgc7sfj0hfneqh37lhmjgnth36mfndwcxkjrakcvpmm727".to_string()
    );

    // account 1
    let addresses = GetAddressesBuilder::new(&secret_manager)
        .with_coin_type(IOTA_COIN_TYPE)
        .with_bech32_hrp(IOTA_BECH32_HRP)
        .with_account_index(1)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "iota1qr43g007shcd7zx3xe7s4lu2c9fr33w7tfjppyy0swlhrxx247szqhuaeaa".to_string()
    );
}

#[tokio::test]
async fn mnemonic_address_generation_shimmer() {
    let mnemonic = "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast";
    let secret_manager = SecretManager::try_from_mnemonic(mnemonic).unwrap();

    // account 0, address 0 and 1
    let addresses = GetAddressesBuilder::new(&secret_manager)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_bech32_hrp(SHIMMER_BECH32_HRP)
        .with_account_index(0)
        .with_range(0..2)
        .finish()
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "smr1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6ckj80y".to_string()
    );
    assert_eq!(
        addresses[1],
        "smr1qznujl7m240za4pf6p0p8rdtqdca6tq7z44heqec8e57xsf429tvz0wt4w3".to_string()
    );

    // account 1
    let addresses = GetAddressesBuilder::new(&secret_manager)
        .with_coin_type(SHIMMER_COIN_TYPE)
        .with_bech32_hrp(SHIMMER_BECH32_HRP)
        .with_account_index(1)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "smr1qrexl2g0m74v57y4kl6kfwqz7zrlrkvjt8m30av0cxgxlu92kyzc5npslm8".to_string()
    );
}

#[tokio::test]
async fn address_generation() {
    #[derive(Serialize, Deserialize)]
    struct AddressData {
        mnemonic: String,
        bech32_hrp: String,
        coin_type: u32,
        account_index: u32,
        internal: bool,
        address_index: u32,
        ed25519_address: String,
        bech32_address: String,
    }

    let file = std::fs::File::open("./tests/fixtures/test_vectors.json").unwrap();
    let json: serde_json::Value = serde_json::from_reader(file).unwrap();
    let general = json.get("general").unwrap();
    let addresses_data: Vec<AddressData> =
        serde_json::from_value(general.get("address_generations").unwrap().clone()).unwrap();

    for address in &addresses_data {
        let secret_manager = SecretManager::try_from_mnemonic(&address.mnemonic).unwrap();
        let addresses = GetAddressesBuilder::new(&secret_manager)
            .with_bech32_hrp(address.bech32_hrp.to_string())
            .with_coin_type(address.coin_type)
            .with_account_index(address.account_index)
            .with_range(address.address_index..address.address_index + 1)
            .with_internal_addresses(address.internal)
            .finish()
            .await
            .unwrap();

        assert_eq!(addresses[0], address.bech32_address);
        if let (_bech32_hrp, Address::Ed25519(ed25519_address)) = Address::try_from_bech32(&addresses[0]).unwrap() {
            assert_eq!(ed25519_address.to_string(), address.ed25519_address);
        } else {
            panic!("Invalid address type")
        }
    }

    #[cfg(feature = "stronghold")]
    for address in &addresses_data {
        let stronghold_filename = format!("{}.stronghold", address.bech32_address);
        let mut stronghold_secret_manager = StrongholdSecretManager::builder()
            .password("some_hopefully_secure_password")
            .build(&stronghold_filename)
            .unwrap();

        stronghold_secret_manager
            .store_mnemonic(address.mnemonic.to_string())
            .await
            .unwrap();

        let addresses = GetAddressesBuilder::new(&SecretManager::Stronghold(stronghold_secret_manager))
            .with_bech32_hrp(address.bech32_hrp.to_string())
            .with_coin_type(address.coin_type)
            .with_account_index(address.account_index)
            .with_range(address.address_index..address.address_index + 1)
            .with_internal_addresses(address.internal)
            .finish()
            .await
            .unwrap();

        assert_eq!(addresses[0], address.bech32_address);
        if let (_bech32_hrp, Address::Ed25519(ed25519_address)) = Address::try_from_bech32(&addresses[0]).unwrap() {
            assert_eq!(ed25519_address.to_string(), address.ed25519_address);
        } else {
            panic!("Invalid address type")
        }
        std::fs::remove_file(stronghold_filename).unwrap();
    }

    #[cfg(feature = "message_interface")]
    {
        let message_handler = message_interface::create_message_handler(None).unwrap();
        for address in &addresses_data {
            let options = GetAddressesBuilderOptions {
                coin_type: Some(address.coin_type),
                account_index: Some(address.account_index),
                range: Some(std::ops::Range {
                    start: address.address_index,
                    end: address.address_index + 1,
                }),
                internal: Some(address.internal),
                bech32_hrp: Some(address.bech32_hrp.to_string()),
                options: None,
            };
            let message = Message::GenerateAddresses {
                secret_manager: SecretManagerDto::Mnemonic(address.mnemonic.clone()),
                options,
            };

            let response = message_handler.send_message(message).await;
            match response {
                Response::GeneratedAddresses(addresses) => {
                    assert_eq!(addresses[0], address.bech32_address);
                    if let (_bech32_hrp, Address::Ed25519(ed25519_address)) =
                        Address::try_from_bech32(&addresses[0]).unwrap()
                    {
                        assert_eq!(ed25519_address.to_string(), address.ed25519_address);
                    } else {
                        panic!("Invalid address type")
                    }
                }
                _ => panic!("Unexpected response type"),
            }
        }
    }

    #[cfg(all(feature = "message_interface", feature = "stronghold"))]
    {
        let message_handler = message_interface::create_message_handler(None).unwrap();
        for address in addresses_data {
            let stronghold_filename = format!("{}.stronghold", address.bech32_address);
            let secret_manager_dto = StrongholdDto {
                password: Some("some_hopefully_secure_password".to_string()),
                timeout: None,
                snapshot_path: stronghold_filename.clone(),
            };
            let message = Message::StoreMnemonic {
                secret_manager: SecretManagerDto::Stronghold(secret_manager_dto.clone()),
                mnemonic: address.mnemonic,
            };
            let _response = message_handler.send_message(message).await;

            let options = GetAddressesBuilderOptions {
                coin_type: Some(address.coin_type),
                account_index: Some(address.account_index),
                range: Some(std::ops::Range {
                    start: address.address_index,
                    end: address.address_index + 1,
                }),
                internal: Some(address.internal),
                bech32_hrp: Some(address.bech32_hrp.to_string()),
                options: None,
            };
            let message = Message::GenerateAddresses {
                secret_manager: SecretManagerDto::Stronghold(secret_manager_dto),
                options,
            };

            let response = message_handler.send_message(message).await;
            match response {
                Response::GeneratedAddresses(addresses) => {
                    assert_eq!(addresses[0], address.bech32_address);
                    if let (_bech32_hrp, Address::Ed25519(ed25519_address)) =
                        Address::try_from_bech32(&addresses[0]).unwrap()
                    {
                        assert_eq!(ed25519_address.to_string(), address.ed25519_address);
                    } else {
                        panic!("Invalid address type")
                    }
                }
                _ => panic!("Unexpected response type"),
            }
            std::fs::remove_file(stronghold_filename).unwrap();
        }
    }
}
