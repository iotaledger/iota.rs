// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{
    api::GetAddressesBuilder,
    constants::{IOTA_BECH32_HRP, IOTA_COIN_TYPE, IOTA_TESTNET_BECH32_HRP, SHIMMER_BECH32_HRP, SHIMMER_COIN_TYPE},
    secret::mnemonic::MnemonicSecretManager,
    Client,
};

#[tokio::test]
async fn addresses() {
    let secret_manager =
        MnemonicSecretManager::try_from_hex_seed("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2")
            .unwrap();

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
    let client = Client::builder().with_offline_mode().finish().await.unwrap();
    let hex_public_key = "2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a";

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
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(mnemonic).unwrap();

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
    let secret_manager = MnemonicSecretManager::try_from_mnemonic(mnemonic).unwrap();

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
