// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::constant::MESSAGE_FRAGMENT_LENGTH;
use bee_signing::ternary::wots::normalize;
use bee_ternary::{t3b1::T3B1Buf, T1B1Buf, TritBuf, Trits, T3B1};

/// Get the maximum bundle hash by selecting the max trytes from all input bundle hashes
pub fn get_max_normalized_bundle_hash(
    bundle_hashes: &[TritBuf<T1B1Buf>],
    security_level: usize,
) -> TritBuf<T1B1Buf> {
    // Normalize the bundle hashes
    let mut normalized_hashes_i8_vecs = bundle_hashes
        .iter()
        .map(|t| {
            TritBuf::<T3B1Buf>::from_i8s(normalize(&t).unwrap().as_i8_slice())
                .unwrap()
                .as_i8_slice()
                .to_vec()
        })
        .collect::<Vec<Vec<i8>>>();

    // Get the max normalized bundle hash
    let mut max_vec_i8 = normalized_hashes_i8_vecs.pop().unwrap();
    while let Some(current_vec_i8) = normalized_hashes_i8_vecs.pop() {
        max_vec_i8 = get_the_max_tryte_values(max_vec_i8, current_vec_i8);
    }

    // Return the max normalized bundle hash in TritBuf::<T1B1Buf>
    unsafe {
        Trits::<T3B1>::from_raw_unchecked(
            &max_vec_i8[..MESSAGE_FRAGMENT_LENGTH * security_level],
            MESSAGE_FRAGMENT_LENGTH * security_level * 3,
        )
        .to_buf::<T3B1Buf>()
        .encode::<T1B1Buf>()
    }
}

/// Get max trytes values from two i8 vectors
pub fn get_the_max_tryte_values(vec_i8_first: Vec<i8>, vec_i8_second: Vec<i8>) -> Vec<i8> {
    vec_i8_first
        .iter()
        .zip(&vec_i8_second)
        .map(|(&x, &y)| x.max(y))
        .collect()
}
