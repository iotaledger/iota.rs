// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_ternary::{T1B1Buf, T3B1Buf, TritBuf, Trits, TryteBuf, T3B1};
use iota_bundle_miner::helper::{get_max_normalized_bundle_hash, get_the_max_tryte_values};
use iota_bundle_miner::miner::trit_buf_to_string;

#[tokio::test]
pub async fn test_get_max_normalized_bundle_hash() {
    let hashes_test = [
        "MDBHGHUGQJCHGYHZENKOHOIZSFN9XYECIYWDVGAAAEBTOGOCVVHKQQBCBWZHYEBJDNWIBQJS9DBVLZGG9",
        "IQRMJSRFJXAAKJQOBELVKKILRGNGGJVWBL9WIJTWYHUUUKNHVZAGEZYRNW9TEPTFZNGZEIILESTGPH9KC",
        "UJAIQGLESRZUEOL9DUZLACRQAFKGXUDGSXCLPICDADVCCAXJSASL9LNFKWVGPLGHERGDIXVKFWULZEQ9C",
    ];
    let max_hash_expected = "NWAIJHLGJJCHKJL9EELLKKILAGK";
    let security_level = 1;
    let hashes_trit_buf_test = hashes_test
        .clone()
        .iter()
        .map(|t| {
            TryteBuf::try_from_str(&(*t).to_string())
                .unwrap()
                .as_trits()
                .encode()
        })
        .collect::<Vec<TritBuf<T1B1Buf>>>();
    let max_hash_computed = get_max_normalized_bundle_hash(&hashes_trit_buf_test, security_level);
    assert_eq!(
        String::from(max_hash_expected),
        trit_buf_to_string(&max_hash_computed).await.unwrap()
    );
}

#[tokio::test]
pub async fn test_get_the_max_tryte_values() {
    let hashes_test = [
        "MDBHGHUGQJCHGYHZENKOHOIZSFN9XYECIYWDVGAAAEBTOGOCVVHKQQBCBWZHYEBJDNWIBQJS9DBVLZGG9",
        "IQRMJSRFJXAAKJQOBELVKKILRGNGGJVWBL9WIJTWYHUUUKNHVZAGEZYRNW9TEPTFZNGZEIILESTGPH9KC",
    ];
    let max_hash_expected =
        "MDBMJHUGJJCHKJHZEELVKKILSGNGGJECIL9DIJAAAHBUUKOHVZHKEZBCBW9HEEBJDNGIEIJLEDBGLHGKC";
    let hashes_tryte_i8_test = hashes_test
        .clone()
        .iter()
        .map(|t| {
            TryteBuf::try_from_str(&(*t).to_string())
                .unwrap()
                .as_trits()
                .encode()
        })
        .collect::<Vec<TritBuf<T3B1Buf>>>();
    let max_hash = get_the_max_tryte_values(
        hashes_tryte_i8_test[0].as_i8_slice().to_vec(),
        hashes_tryte_i8_test[1].as_i8_slice().to_vec(),
    );
    let max_hash_computed = unsafe {
        Trits::<T3B1>::from_raw_unchecked(&max_hash, max_hash.len() * 3)
            .to_buf::<T3B1Buf>()
            .encode::<T1B1Buf>()
    };
    assert_eq!(
        String::from(max_hash_expected),
        trit_buf_to_string(&max_hash_computed).await.unwrap()
    );
}
