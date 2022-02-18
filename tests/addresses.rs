// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::GetAddressesBuilder, signing::mnemonic::MnemonicSigner, Client};

#[tokio::test]
async fn addresses() {
    let signer =
        MnemonicSigner::new_from_seed("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap();
    let addresses = GetAddressesBuilder::new(&signer)
        .with_bech32_hrp("atoi".into())
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
    let iota = Client::builder().with_offline_mode().finish().await.unwrap();

    let hex_public_key = "2baaf3bca8ace9f862e60184bd3e79df25ff230f7eaaa4c7f03daa9833ba854a";

    let public_key_address = iota
        .hex_public_key_to_bech32_address(hex_public_key, Some("atoi"))
        .await
        .unwrap();
    assert_eq!(
        public_key_address,
        "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string()
    );
}

#[tokio::test]
async fn mnemonic_address_generation() {
    let mnemonic = "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast";
    let signer = MnemonicSigner::new(mnemonic).unwrap();

    let addresses = GetAddressesBuilder::new(&signer)
        .with_bech32_hrp("iota".into())
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "iota1qpg2xkj66wwgn8p2ggnp7p582gj8g6p79us5hve2tsudzpsr2ap4skprwjg".to_string()
    );
}
