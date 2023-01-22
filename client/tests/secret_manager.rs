// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{api::GetAddressesBuilder, constants::SHIMMER_TESTNET_BECH32_HRP, secret::SecretManager, Result};

#[tokio::test]
async fn mnemonic_secret_manager_dto() -> Result<()> {
    let dto = r#"{"mnemonic": "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast"}"#;
    let secret_manager: SecretManager = dto.parse()?;

    let addresses = GetAddressesBuilder::new(&secret_manager)
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
    let dto = r#"{"stronghold": {"password": "some_hopefully_secure_password", "snapshotPath": "snapshot_test_dir/test.stronghold"}}"#;
    let mnemonic = String::from(
        "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast",
    );

    let mut secret_manager: SecretManager = dto.parse()?;

    // The mnemonic only needs to be stored the first time
    if let SecretManager::Stronghold(secret_manager) = &mut secret_manager {
        secret_manager.store_mnemonic(mnemonic.clone()).await.unwrap();
    } else {
        panic!("expect a Stronghold secret manager, but it's not the case!");
    }

    let addresses = GetAddressesBuilder::new(&secret_manager)
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
    if let SecretManager::Stronghold(secret_manager) = &mut secret_manager {
        assert!(secret_manager.store_mnemonic(mnemonic).await.is_err());
    } else {
        panic!("expect a Stronghold secret manager, but it's not the case!");
    }

    // Remove garbage after test, but don't care about the result
    std::fs::remove_dir_all("snapshot_test_dir").unwrap_or(());
    Ok(())
}

#[cfg(feature = "stronghold")]
#[tokio::test]
async fn stronghold_mnemonic_missing() -> Result<()> {
    // Cleanup of a possibly failed run
    std::fs::remove_dir_all("stronghold_mnemonic_missing").unwrap_or(());

    let stronghold_secret_manager = iota_client::secret::stronghold::StrongholdSecretManager::builder()
        .password("some_hopefully_secure_password")
        .build("stronghold_mnemonic_missing/test.stronghold")?;

    // Generating addresses will fail because no mnemonic has been stored
    let error = GetAddressesBuilder::new(&SecretManager::Stronghold(stronghold_secret_manager))
        // .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP)
        // .with_coin_type(iota_client::constants::SHIMMER_COIN_TYPE)
        .finish()
        .await
        .unwrap_err();

    match error {
        iota_client::Error::StrongholdMnemonicMissing => {}
        _ => panic!("expected StrongholdMnemonicMissing error"),
    }

    // Remove garbage after test, but don't care about the result
    std::fs::remove_dir_all("stronghold_mnemonic_missing").unwrap_or(());
    Ok(())
}
