// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{
    api::GetAddressesBuilder,
    constants::SHIMMER_TESTNET_BECH32_HRP,
    signing::{types::SignerTypeDto, SignerHandle},
    Result,
};

#[tokio::test]
async fn mnemonic_signer_dto() -> Result<()> {
    let mnemonic = "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast".to_string();
    let signer_type_dto = SignerTypeDto::Mnemonic(mnemonic);
    let signer = SignerHandle::from_str(&serde_json::to_string(&signer_type_dto)?)?;

    let addresses = GetAddressesBuilder::new(&signer)
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP.to_string())
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
async fn stronghold_signer_dto() -> Result<()> {
    let stronghold_dto_str =
        r#"{ "Stronghold": { "password": "some_hopefully_secure_password", "snapshotPath": "test.stronghold" } }"#;
    let mnemonic = "acoustic trophy damage hint search taste love bicycle foster cradle brown govern endless depend situate athlete pudding blame question genius transfer van random vast".to_string();

    let signer_type_dto: SignerTypeDto = serde_json::from_str(stronghold_dto_str)?;
    let stronghold_signer = SignerHandle::from_str(&serde_json::to_string(&signer_type_dto)?)?;

    // The mnemonic only needs to be stored the first time
    stronghold_signer
        .lock()
        .await
        .store_mnemonic(mnemonic.clone())
        .await
        .unwrap();

    let addresses = GetAddressesBuilder::new(&stronghold_signer)
        .with_bech32_hrp(SHIMMER_TESTNET_BECH32_HRP.to_string())
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
    assert!(stronghold_signer.lock().await.store_mnemonic(mnemonic).await.is_err());

    // Remove garbage after test, but don't care about the result
    std::fs::remove_file("test.stronghold").unwrap_or(());
    Ok(())
}
