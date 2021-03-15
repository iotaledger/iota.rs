// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::GetAddressesBuilder, Seed};

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
    assert_eq!(addresses[0].1, false);
    assert_eq!(
        *addresses[1].0,
        "atoi1qprxpfvaz2peggq6f8k9cj8zfsxuw69e4nszjyv5kuf8yt70t2847shpjak".to_string()
    );
    assert_eq!(addresses[1].1, true);
}
