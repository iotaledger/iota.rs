// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_ternary::{T1B1Buf, TritBuf, TryteBuf};
use iota_bundle_miner::miner::{
    absorb_and_get_normalized_bundle_hash, create_obsolete_tag, increase_essense,
    mining_worker_with_non_crack_probability_stop_criteria, prepare_keccak_384, trit_buf_to_string,
    update_essense_with_new_obsolete_tag, MinerBuilder, MinerEvent, StopMiningCriteria,
    EQUAL_TRAGET_HASH, LESS_THAN_MAX_HASH,
};

#[tokio::test]
pub async fn test_obsolete_tag_creation() {
    let increment: i64 = 3;
    let worker_id: i32 = 0;
    let essences = vec![
        "EDIKZYSKVIWNNTMKWUSXKFMYQVIMBNECNYKBG9YVRKUMXNIXSVAKTIDCAHULLLXR9FSQSDDOFOJWKFACD",
        "A99999999999999999999999999999999999999999999999999999999999999999999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "Z99999999999999999999999999999999999999999999999999999999999999A99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999B99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999C99999999C99999999",
    ];
    let target_hash =
        "NNNNNNFAHTZDAMSFMGDCKRWIMMVPVISUYXKTFADURMAEMTNFGBUMODCKQZPMWHUGISUOCWQQL99ZTGCJD";
    let kerl = prepare_keccak_384(
        &essences[..essences.len() - 1]
            .iter()
            .map(|t| {
                TryteBuf::try_from_str(&t.to_string())
                    .unwrap()
                    .as_trits()
                    .encode()
            })
            .collect::<Vec<TritBuf<T1B1Buf>>>(),
    )
    .await;
    let mut last_essence: TritBuf<T1B1Buf> = TryteBuf::try_from_str(essences[essences.len() - 1])
        .unwrap()
        .as_trits()
        .encode();

    let obselete_tag = create_obsolete_tag(increment, worker_id).await;
    last_essence = update_essense_with_new_obsolete_tag(last_essence, &obselete_tag).await;
    let hash = absorb_and_get_normalized_bundle_hash(kerl, &last_essence).await;

    let hash_str = trit_buf_to_string(&hash).await.unwrap();
    assert_eq!(String::from(target_hash), hash_str);
}

#[tokio::test]
pub async fn test_obsolete_tag_increment() {
    let increment: i64 = 0;
    let worker_id: i32 = 0;
    let essences = vec![
        "EDIKZYSKVIWNNTMKWUSXKFMYQVIMBNECNYKBG9YVRKUMXNIXSVAKTIDCAHULLLXR9FSQSDDOFOJWKFACD",
        "A99999999999999999999999999999999999999999999999999999999999999999999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "Z99999999999999999999999999999999999999999999999999999999999999A99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999B99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999C99999999C99999999",
    ];
    let target_hash =
        "NNNNNNFAHTZDAMSFMGDCKRWIMMVPVISUYXKTFADURMAEMTNFGBUMODCKQZPMWHUGISUOCWQQL99ZTGCJD";
    let kerl = prepare_keccak_384(
        &essences[..essences.len() - 1]
            .iter()
            .map(|t| {
                TryteBuf::try_from_str(&t.to_string())
                    .unwrap()
                    .as_trits()
                    .encode()
            })
            .collect::<Vec<TritBuf<T1B1Buf>>>(),
    )
    .await;
    let mut last_essence: TritBuf<T1B1Buf> = TryteBuf::try_from_str(essences[essences.len() - 1])
        .unwrap()
        .as_trits()
        .encode();

    let obselete_tag = create_obsolete_tag(increment, worker_id).await;
    last_essence = update_essense_with_new_obsolete_tag(last_essence, &obselete_tag).await;
    let last_essence = increase_essense(last_essence).await.unwrap();
    let last_essence = increase_essense(last_essence).await.unwrap();
    let last_essence = increase_essense(last_essence).await.unwrap();
    let hash = absorb_and_get_normalized_bundle_hash(kerl, &last_essence).await;

    let hash_str = trit_buf_to_string(&hash).await.unwrap();
    assert_eq!(String::from(target_hash), hash_str);
}

#[tokio::test]
pub async fn test_worker() {
    let increment: i64 = 0;
    let worker_id: usize = 0;
    let essences = vec![
        "EDIKZYSKVIWNNTMKWUSXKFMYQVIMBNECNYKBG9YVRKUMXNIXSVAKTIDCAHULLLXR9FSQSDDOFOJWKFACD",
        "A99999999999999999999999999999999999999999999999999999999999999999999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "Z99999999999999999999999999999999999999999999999999999999999999A99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999B99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999C99999999C99999999",
    ];
    let target_hash =
        "NNNNNNFAHTZDAMSFMGDCKRWIMMVPVISUYXKTFADURMAEMTNFGBUMODCKQZPMWHUGISUOCWQQL99ZTGCJD";

    mining_worker_with_non_crack_probability_stop_criteria(
        increment,
        worker_id,
        essences
            .iter()
            .map(|t| {
                TryteBuf::try_from_str(&t.to_string())
                    .unwrap()
                    .as_trits()
                    .encode()
            })
            .collect::<Vec<TritBuf<T1B1Buf>>>(),
        TryteBuf::try_from_str(&target_hash.to_string())
            .unwrap()
            .as_trits()
            .encode(),
        EQUAL_TRAGET_HASH,
    )
    .await;
}

#[test]
pub fn test_equal_target_hash_criterion() {
    let mined_hash = "NWAIJHLGJJCHKJL9EELLKKILAGK";
    let target_hash_true = "NWAIJHLGJJCHKJL9EELLKKILAGK";
    let target_hash_false = "NWAIJHLGJJCHK9L9EELLKKILAGK";
    let target_hash_true_trit_buf = TryteBuf::try_from_str(&target_hash_true.to_string())
        .unwrap()
        .as_trits()
        .encode();
    let target_hash_false_trit_buf = TryteBuf::try_from_str(&target_hash_false.to_string())
        .unwrap()
        .as_trits()
        .encode();
    let mined_hash_trit_buf = TryteBuf::try_from_str(&mined_hash.to_string())
        .unwrap()
        .as_trits()
        .encode();
    assert_eq!(
        true,
        LESS_THAN_MAX_HASH
            .judge(&mined_hash_trit_buf, &target_hash_true_trit_buf)
            .unwrap()
    );
    assert_eq!(
        false,
        LESS_THAN_MAX_HASH
            .judge(&mined_hash_trit_buf, &target_hash_false_trit_buf)
            .unwrap()
    );
}

#[test]
pub fn test_less_than_max_hash_criterion() {
    let max_hash_true = "NWAIJHLGJJCHKJL9EELLKKILAGK";
    let max_hash_false = "NOPGBAGZHGZBJZHUC9TR9HFNTFE";
    let mined_hash =
        "NOPGBAGZHGZBJZHUC9TR9HFOTFEZXOCUJOUXVVMXMB9JJTYKGLOATSMMMJNU9IQHSWVEHBKOONQAZENGB";
    let max_hash_true_trit_buf = TryteBuf::try_from_str(&max_hash_true.to_string())
        .unwrap()
        .as_trits()
        .encode();
    let max_hash_false_trit_buf = TryteBuf::try_from_str(&max_hash_false.to_string())
        .unwrap()
        .as_trits()
        .encode();
    let mined_hash_trit_buf = TryteBuf::try_from_str(&mined_hash.to_string())
        .unwrap()
        .as_trits()
        .encode();
    assert_eq!(
        true,
        LESS_THAN_MAX_HASH
            .judge(&mined_hash_trit_buf, &max_hash_true_trit_buf)
            .unwrap()
    );
    assert_eq!(
        false,
        LESS_THAN_MAX_HASH
            .judge(&mined_hash_trit_buf, &max_hash_false_trit_buf)
            .unwrap()
    );
}

#[test]
pub fn test_miner_builder() {
    let essences = vec![
        "EDIKZYSKVIWNNTMKWUSXKFMYQVIMBNECNYKBG9YVRKUMXNIXSVAKTIDCAHULLLXR9FSQSDDOFOJWKFACD",
        "A99999999999999999999999999999999999999999999999999999999999999999999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "Z99999999999999999999999999999999999999999999999999999999999999A99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999B99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999C99999999C99999999",
    ];

    let _ = MinerBuilder::new()
        .with_known_bundle_hashes(vec![])
        .with_core_thread_count(1)
        .with_worker_count(5)
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
        .with_mining_timeout(10)
        .finish()
        .unwrap();
}

#[test]
pub fn test_miner_equal_target_hash_run() {
    let essences = vec![
        "EDIKZYSKVIWNNTMKWUSXKFMYQVIMBNECNYKBG9YVRKUMXNIXSVAKTIDCAHULLLXR9FSQSDDOFOJWKFACD",
        "A99999999999999999999999999999999999999999999999999999999999999999999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "Z99999999999999999999999999999999999999999999999999999999999999A99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999B99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999C99999999C99999999",
    ];
    let target_hash =
        "NNNNNNFAHTZDAMSFMGDCKRWIMMVPVISUYXKTFADURMAEMTNFGBUMODCKQZPMWHUGISUOCWQQL99ZTGCJD";
    let expected_essence =
        "999999999999999999999999999C99999999999999999999999999999999999C99999999C99999999";
    let expected_essence: TritBuf<T1B1Buf> = TryteBuf::try_from_str(&expected_essence.to_string())
        .unwrap()
        .as_trits()
        .encode();
    let mut miner = MinerBuilder::new()
        .with_known_bundle_hashes(vec![])
        .with_core_thread_count(1)
        .with_worker_count(5)
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
        .with_mining_timeout(10)
        .finish()
        .unwrap();
    if let MinerEvent::MinedEssence(mined_essence) = miner
        .run_with_with_non_crack_probability_stop_criteria(
            TryteBuf::try_from_str(&target_hash.to_string())
                .unwrap()
                .as_trits()
                .encode(),
            EQUAL_TRAGET_HASH,
        )
    {
        assert_eq!(mined_essence, expected_essence);
    } else {
        panic!();
    }
}

#[test]
pub fn test_miner_less_than_max_hash_run() {
    let essences = vec![
        "EDIKZYSKVIWNNTMKWUSXKFMYQVIMBNECNYKBG9YVRKUMXNIXSVAKTIDCAHULLLXR9FSQSDDOFOJWKFACD",
        "A99999999999999999999999999999999999999999999999999999999999999999999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "Z99999999999999999999999999999999999999999999999999999999999999A99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999B99999999C99999999",
        "BMLAF9QKVBYJTGHTGFFNOVDTGEMA9MSXGTJYSRRHEYTMMKRMQYETPJVAADGYLPYMGBJERKLJVUZUZYRQD",
        "999999999999999999999999999999999999999999999999999999999999999C99999999C99999999",
    ];
    let target_hash =
        "NNNNNNFAHTZDAMSFMGDCKRWIMMVPVISUYXKTFADURMAEMTNFGBUMODCKQZPMWHUGISUOCWQQL99ZTGCJD";
    let expected_essence =
        "999999999999999999999999999C99999999999999999999999999999999999C99999999C99999999";
    let expected_essence: TritBuf<T1B1Buf> = TryteBuf::try_from_str(&expected_essence.to_string())
        .unwrap()
        .as_trits()
        .encode();
    let mut miner = MinerBuilder::new()
        .with_known_bundle_hashes(vec![])
        .with_core_thread_count(1)
        .with_worker_count(5)
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
        .with_mining_timeout(10)
        .finish()
        .unwrap();
    if let MinerEvent::MinedEssence(mined_essence) = miner
        .run_with_with_non_crack_probability_stop_criteria(
            TryteBuf::try_from_str(&target_hash.to_string())
                .unwrap()
                .as_trits()
                .encode(),
            LESS_THAN_MAX_HASH,
        )
    {
        assert_eq!(mined_essence, expected_essence);
    } else {
        panic!();
    }
}
