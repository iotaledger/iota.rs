// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

mod alias;
mod ed25519;
mod nft;

use iota_types::block::{address::Address, Error};

const ED25519_ADDRESS_INVALID: &str = "0x52fdfc072182654f163f5f0f9a621d729566c74d10037c4d7bbb0407d1e2c64x";

#[test]
fn invalid_bech32() {
    let address = Address::try_from_bech32(ED25519_ADDRESS_INVALID);

    assert!(matches!(address, Err(Error::InvalidAddress)));
}
