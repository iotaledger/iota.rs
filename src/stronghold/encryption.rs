// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! A symmetric encryption implementation for `StrongholdAdapter`.

use crate::Result;
use crypto::ciphers::{chacha::XChaCha20Poly1305, traits::Aead};

// Fixed position indexes for referencing the concatenated parts in a ciphertext:
//
//         POS_TAG_END v POS_CIPHERTEXT_START
//       +-------+-----+------------+
//     0 | Nonce | Tag | Ciphertext | v.len()
//       +-------+-----+------------+
// POS_NONCE_END ^ POS_TAG_START
//
// This layout is how it's been historically.
const POS_NONCE_END: usize = XChaCha20Poly1305::NONCE_LENGTH;
const POS_TAG_START: usize = POS_NONCE_END;
const POS_TAG_END: usize = XChaCha20Poly1305::NONCE_LENGTH + XChaCha20Poly1305::TAG_LENGTH;
const POS_CIPHERTEXT_START: usize = POS_TAG_END;

pub(super) fn encrypt(plaintext: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let mut nonce = [0u8; XChaCha20Poly1305::NONCE_LENGTH];
    let mut tag = vec![0u8; XChaCha20Poly1305::TAG_LENGTH];
    let mut ciphertext = vec![0u8; plaintext.len()];

    crypto::utils::rand::fill(&mut nonce)?;

    XChaCha20Poly1305::encrypt(
        key.try_into().unwrap(),
        &nonce.try_into().unwrap(),
        &[],
        plaintext,
        ciphertext.as_mut(),
        tag.as_mut_slice().try_into().unwrap(),
    )?;

    let mut ret = nonce.to_vec();
    ret.append(&mut tag);
    ret.append(&mut ciphertext);

    Ok(ret)
}

pub(super) fn decrypt(data: &[u8], key: &[u8]) -> Result<Vec<u8>> {
    let nonce = &data[..POS_NONCE_END];
    let tag = &data[POS_TAG_START..POS_TAG_END];
    let ciphertext = &data[POS_CIPHERTEXT_START..];

    let mut plaintext = vec![0u8; ciphertext.len()];

    XChaCha20Poly1305::decrypt(
        key.try_into().unwrap(),
        nonce.try_into().unwrap(),
        &[],
        &mut plaintext,
        ciphertext,
        tag.try_into().unwrap(),
    )?;

    Ok(plaintext)
}
