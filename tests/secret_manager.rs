// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::GetAddressesBuilder, constants::SHIMMER_TESTNET_BECH32_HRP, secret::SecretManagerType, Result};

#[tokio::test]
async fn mnemonic_secret_manager_dto() -> Result<()> {
    let dto = r#"{"Mnemonic": "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast"}"#;

    let secmngr = match dto.parse::<SecretManagerType>()? {
        SecretManagerType::Mnemonic(secmngr) => *secmngr,
        _ => panic!(),
    };

    let addresses = GetAddressesBuilder::new(&secmngr)
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a".to_string()
    );

    Ok(())
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold_secret_manager_dto() -> Result<()> {
    let dto = r#"{"Stronghold": {"password": "some_hopefully_secure_password", "snapshotPath": "test.stronghold"}}"#;
    let mnemonic = String::from(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast",
    );

    let mut secmngr = match dto.parse::<SecretManagerType>()? {
        SecretManagerType::Stronghold(secmngr) => *secmngr,
        _ => panic!(),
    };

    // The mnemonic only needs to be stored the first time
    secmngr.store_mnemonic(mnemonic.clone()).await.unwrap();

    let addresses = GetAddressesBuilder::new(&secmngr)
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        .with_account_index(0)
        .with_range(0..1)
        .finish()
        .await
        .unwrap();

    assert_eq!(
        addresses[0],
        "rms1qzev36lk0gzld0k28fd2fauz26qqzh4hd4cwymlqlv96x7phjxcw6v3ea5a".to_string()
    );

    // Calling store_mnemonic() twice should fail, because we would otherwise overwrite the stored entry
    assert!(secmngr.store_mnemonic(mnemonic).await.is_err());

    // Remove garbage after test, but don't care about the result
    std::fs::remove_file("test.stronghold").unwrap_or(());
    Ok(())
}
