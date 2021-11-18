// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::GetAddressesBuilder, Client, Seed};

#[tokio::test]
async fn addresses() {
    let seed =
        Seed::from_bytes(&hex::decode("256a818b2aac458941f7274985a410e57fb750f3a3a67969ece5bd9ae7eef5b2").unwrap());
    let addresses = GetAddressesBuilder::new(&seed)
        .with_bech32_hrp("atoi".into())
        .with_account_index(0)
        .with_range(0..1)
        .get_all()
        .await
        .unwrap();
    assert_eq!(
        *addresses[0].0,
        "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".to_string()
    );
    assert!(!addresses[0].1);
    assert_eq!(
        *addresses[1].0,
        "atoi1qprxpfvaz2peggq6f8k9cj8zfsxuw69e4nszjyv5kuf8yt70t2847shpjak".to_string()
    );
    assert!(addresses[1].1);
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
