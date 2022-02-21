// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{
    api::GetAddressesBuilder,
    signing::{types::SignerTypeDto, SignerHandle},
    Result,
};

#[tokio::test]
async fn signer_dto() -> Result<()> {
    let mnemonic = "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast".to_string();
    let signer_type_dto = SignerTypeDto::Mnemonic(mnemonic);
    let signer = SignerHandle::from_str(&serde_json::to_string(&signer_type_dto)?)?;

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
    Ok(())
}
