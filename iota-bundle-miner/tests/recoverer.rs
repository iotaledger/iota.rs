// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_ternary::{T1B1Buf, TritBuf, TryteBuf};
use iota_bundle_miner::{
    miner::{CrackabilityMinerEvent, MinerBuilder},
    recoverer::{get_crack_probability, RecovererBuilder},
};
use std::fs::File;
use std::io::BufReader;

extern crate serde;
extern crate serde_json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CrackabilityTestCase {
    hash: String,
    score: f64,
    security: usize,
}

#[test]
pub fn test_get_crack_probability_by_generated_patterns() {
    // Read the test vector
    let file = File::open("./tests/crackability.json").unwrap();
    let reader = BufReader::new(file);
    let test_vector: Vec<CrackabilityTestCase> = serde_json::from_reader(reader).unwrap();

    for pattern in test_vector {
        let hashes_tests = vec![pattern.hash];
        let p_expected = pattern.score;
        let security_level = pattern.security;

        let hashes_trit_i8_test = hashes_tests
            .clone()
            .iter()
            .map(|t| {
                TryteBuf::try_from_str(&(*t).to_string())
                    .unwrap()
                    .as_trits()
                    .encode()
            })
            .collect::<Vec<TritBuf<T1B1Buf>>>();
        let p_actual = get_crack_probability(security_level, &hashes_trit_i8_test);
        println!("p_actual: {:?}, p_expected: {:?}", p_actual, p_expected);
        assert_eq!(true, (p_expected - p_actual).abs() < p_expected * 1e-9);
    }
}

#[test]
pub fn test_get_crack_probability_security_1() {
    let hashes_test =
        ["SEYZLVFTIKFROANWJDVJVOU9HZCHSHOZEIKS9CGHNHGCRUJBUEAQPBYWREUEXEAIRDXEWO9H9HXRIWVKB"];
    let hashes_trit_i8_test = hashes_test
        .clone()
        .iter()
        .map(|t| {
            TryteBuf::try_from_str(&(*t).to_string())
                .unwrap()
                .as_trits()
                .encode()
        })
        .collect::<Vec<TritBuf<T1B1Buf>>>();
    let security_level = 1;
    let p_actual = get_crack_probability(security_level, &hashes_trit_i8_test);
    let p_expected = 9.77707241082429544488650258100839580e-15_f64;
    assert_eq!(true, (p_expected - p_actual).abs() < p_expected * 1e-9);
}

#[test]
pub fn test_get_crack_probability_security_2() {
    let hashes_test =
        ["SEYZLVFTIKFROANWJDVJVOU9HZCHSHOZEIKS9CGHNHGCRUJBUEAQPBYWREUEXEAIRDXEWO9H9HXRIWVKB"];
    let hashes_trit_i8_test = hashes_test
        .clone()
        .iter()
        .map(|t| {
            TryteBuf::try_from_str(&(*t).to_string())
                .unwrap()
                .as_trits()
                .encode()
        })
        .collect::<Vec<TritBuf<T1B1Buf>>>();
    let security_level = 2;
    let p_actual = get_crack_probability(security_level, &hashes_trit_i8_test);
    let p_expected = 6.3270996534167465314983e-28_f64;
    assert_eq!(true, (p_expected - p_actual).abs() < p_expected * 1e-9);
}

#[test]
pub fn test_get_crack_probability_security_3() {
    let hashes_test =
        ["SEYZLVFTIKFROANWJDVJVOU9HZCHSHOZEIKS9CGHNHGCRUJBUEAQPBYWREUEXEAIRDXEWO9H9HXRIWVKB"];
    let hashes_trit_i8_test = hashes_test
        .clone()
        .iter()
        .map(|t| {
            TryteBuf::try_from_str(&(*t).to_string())
                .unwrap()
                .as_trits()
                .encode()
        })
        .collect::<Vec<TritBuf<T1B1Buf>>>();
    let security_level = 3;
    let p_actual = get_crack_probability(security_level, &hashes_trit_i8_test);
    let p_expected = 1.1704004458e-40_f64;
    assert_eq!(true, (p_expected - p_actual).abs() < p_expected * 1e-9);
}
#[tokio::test]
pub async fn test_recoverer_run_security_1() {
    let known_bundle_hashes =
        vec!["SEYZLVFTIKFROANWJDVJVOU9HZCHSHOZEIKS9CGHNHGCRUJBUEAQPBYWREUEXEAIRDXEWO9H9HXRIWVKB"];
    let essences = vec![
        "GPB9PBNCJTPGFZ9CCAOPCZBFMBSMMFMARZAKBMJFMTSECEBRWMGLPTYZRAFKUFOGJQVWVUPPABLTTLCIA",
        "A99999999999999999999999999999999999999999999999999999999999999999999999A99999999",
        "GMLRCFYRCWPZTORXSFCEGKXTVQGPFI9W9EJLERYJMEJGIPLNCLIKCCAOKQEFYUYCEUGIZKCSSJL9JD9SC",
        "Z99999999999999999999999999999999999999999999999999999999999999A99999999A99999999",
    ];
    let security_level: usize = 1;
    let mined_iteration_expected: usize = 185;
    let mined_crackability_expected: f64 = 8.99389659655018e-9;
    let miner = MinerBuilder::new()
        .with_known_bundle_hashes(
            known_bundle_hashes
                .clone()
                .iter()
                .map(|t| {
                    TryteBuf::try_from_str(&(*t).to_string())
                        .unwrap()
                        .as_trits()
                        .encode()
                })
                .collect::<Vec<TritBuf<T1B1Buf>>>(),
        )
        .with_security_level(security_level)
        .with_core_thread_count(1)
        .with_worker_count(1)
        .with_essences_from_unsigned_bundle(
            essences
                .clone()
                .iter()
                .map(|t| {
                    TryteBuf::try_from_str(&(*t).to_string())
                        .unwrap()
                        .as_trits()
                        .encode()
                })
                .collect::<Vec<TritBuf<T1B1Buf>>>(),
        )
        .with_mining_timeout(20)
        .finish()
        .unwrap();
    let mut recoverer = RecovererBuilder::new()
        .with_security_level(security_level)
        .with_known_bundle_hashes(
            known_bundle_hashes
                .clone()
                .iter()
                .map(|t| {
                    TryteBuf::try_from_str(&(*t).to_string())
                        .unwrap()
                        .as_trits()
                        .encode()
                })
                .collect::<Vec<TritBuf<T1B1Buf>>>(),
        )
        .with_threshold(1e-8_f64)
        .miner(miner)
        .finish()
        .unwrap();

    if let CrackabilityMinerEvent::MinerInfo(mined_info) = recoverer.recover().await {
        assert_eq!(mined_iteration_expected, mined_info.mined_iteration);
        assert_eq!(
            true,
            (mined_crackability_expected - mined_info.crackability).abs()
                < mined_crackability_expected * 1e-9
        );
    } else {
        panic!();
    }
}

#[tokio::test]
pub async fn test_recoverer_run_security_2() {
    let known_bundle_hashes =
        vec!["SEYZLVFTIKFROANWJDVJVOU9HZCHSHOZEIKS9CGHNHGCRUJBUEAQPBYWREUEXEAIRDXEWO9H9HXRIWVKB"];
    let essences = vec![
        "GPB9PBNCJTPGFZ9CCAOPCZBFMBSMMFMARZAKBMJFMTSECEBRWMGLPTYZRAFKUFOGJQVWVUPPABLTTLCIA",
        "A99999999999999999999999999999999999999999999999999999999999999999999999B99999999",
        "GMLRCFYRCWPZTORXSFCEGKXTVQGPFI9W9EJLERYJMEJGIPLNCLIKCCAOKQEFYUYCEUGIZKCSSJL9JD9SC",
        "Z99999999999999999999999999999999999999999999999999999999999999A99999999B99999999",
        "GMLRCFYRCWPZTORXSFCEGKXTVQGPFI9W9EJLERYJMEJGIPLNCLIKCCAOKQEFYUYCEUGIZKCSSJL9JD9SC",
        "999999999999999999999999999999999999999999999999999999999999999B99999999B99999999",
    ];
    let security_level: usize = 2;
    let mined_iteration_expected: usize = 28925;
    let mined_crackability_expected: f64 = 9.2430303968744906557891424497679297e-16;
    let miner = MinerBuilder::new()
        .with_known_bundle_hashes(
            known_bundle_hashes
                .clone()
                .iter()
                .map(|t| {
                    TryteBuf::try_from_str(&(*t).to_string())
                        .unwrap()
                        .as_trits()
                        .encode()
                })
                .collect::<Vec<TritBuf<T1B1Buf>>>(),
        )
        .with_security_level(security_level)
        .with_core_thread_count(1)
        .with_worker_count(1)
        .with_essences_from_unsigned_bundle(
            essences
                .clone()
                .iter()
                .map(|t| {
                    TryteBuf::try_from_str(&(*t).to_string())
                        .unwrap()
                        .as_trits()
                        .encode()
                })
                .collect::<Vec<TritBuf<T1B1Buf>>>(),
        )
        .with_mining_timeout(30)
        .finish()
        .unwrap();
    let mut recoverer = RecovererBuilder::new()
        .with_security_level(security_level)
        .with_known_bundle_hashes(
            known_bundle_hashes
                .clone()
                .iter()
                .map(|t| {
                    TryteBuf::try_from_str(&(*t).to_string())
                        .unwrap()
                        .as_trits()
                        .encode()
                })
                .collect::<Vec<TritBuf<T1B1Buf>>>(),
        )
        .with_threshold(1e-15_f64)
        .miner(miner)
        .finish()
        .unwrap();

    if let CrackabilityMinerEvent::MinerInfo(mined_info) = recoverer.recover().await {
        assert_eq!(mined_iteration_expected, mined_info.mined_iteration);
        assert_eq!(
            true,
            (mined_crackability_expected - mined_info.crackability).abs()
                < mined_crackability_expected * 1e-9
        );
    } else {
        panic!();
    }
}
