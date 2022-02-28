// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{Client, Result};

#[tokio::test]
async fn mnemonic() -> Result<()> {
    let mnemonic = Client::generate_mnemonic()?;
    assert!(Client::mnemonic_to_hex_seed(&mnemonic).is_ok());
    assert!(Client::mnemonic_to_hex_seed("until fire hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket").is_ok());
    assert!(Client::mnemonic_to_hex_seed("fire until hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket").is_err());
    assert!(Client::mnemonic_to_hex_seed("invalid mnemonic").is_err());
    // mnemonic with space at the beginning or end should return the same as without
    let mnemonic = "until fire hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket";
    let mnemonic_with_spaces = " until fire hat mountain zoo grocery real deny advance change marble taste goat ivory wheat bubble panic banner tattoo client ticket action race rocket ";
    assert_eq!(
        Client::mnemonic_to_hex_seed(mnemonic).unwrap(),
        Client::mnemonic_to_hex_seed(mnemonic_with_spaces).unwrap()
    );
    Ok(())
}
