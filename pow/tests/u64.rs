// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_pow::providers::NonceProvider;
use iota_types::block::rand::bytes::rand_bytes;

#[test]
fn constant_provide() {
    let bytes = rand_bytes(256);
    let nonce_1 = 42;
    let nonce_2 = nonce_1.nonce(&bytes[0..248], 4000).unwrap();

    assert_eq!(nonce_1, nonce_2);
}
