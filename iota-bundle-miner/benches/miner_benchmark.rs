// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use bee_ternary::TryteBuf;
use bee_ternary::{T1B1Buf, TritBuf};
use criterion::{criterion_group, criterion_main, Criterion};
use iota_bundle_miner::miner::{
    absorb_and_get_normalized_bundle_hash, create_obsolete_tag, increase_essense,
    mining_worker_with_non_crack_probability_stop_criteria as worker, prepare_keccak_384,
    update_essense_with_new_obsolete_tag, EqualTargetHash,
};
use tokio::runtime::Runtime;

pub fn obsolete_tag_creation() {
    // Create the runtime
    let mut rt = Runtime::new().unwrap();

    // Execute the future, blocking the current thread until completion
    rt.block_on(async {
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
        let mut last_essence: TritBuf<T1B1Buf> =
            TryteBuf::try_from_str(essences[essences.len() - 1])
                .unwrap()
                .as_trits()
                .encode();

        let obselete_tag = create_obsolete_tag(increment, worker_id).await;
        last_essence = update_essense_with_new_obsolete_tag(last_essence, &obselete_tag).await;
        let _hash = absorb_and_get_normalized_bundle_hash(kerl, &last_essence).await;
    });
}

pub fn obsolete_tag_increment() {
    // Create the runtime
    let mut rt = Runtime::new().unwrap();

    // Execute the future, blocking the current thread until completion
    rt.block_on(async {
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
        let mut last_essence: TritBuf<T1B1Buf> =
            TryteBuf::try_from_str(essences[essences.len() - 1])
                .unwrap()
                .as_trits()
                .encode();

        let obselete_tag = create_obsolete_tag(increment, worker_id).await;
        last_essence = update_essense_with_new_obsolete_tag(last_essence, &obselete_tag).await;
        let last_essence = increase_essense(last_essence).await.unwrap();
        let last_essence = increase_essense(last_essence).await.unwrap();
        let last_essence = increase_essense(last_essence).await.unwrap();
        let _hash = absorb_and_get_normalized_bundle_hash(kerl, &last_essence).await;
    });
}

pub fn mining_worker() {
    // Create the runtime
    let mut rt = Runtime::new().unwrap();

    // Execute the future, blocking the current thread until completion
    rt.block_on(async {
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
        worker(
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
            EqualTargetHash,
        )
        .await;
    });
}

fn bench_tag_creation(c: &mut Criterion) {
    c.bench_function("obsolete_tag_creation", |b| {
        b.iter(|| obsolete_tag_creation())
    });
}

fn bench_obsolete_tag_increment(c: &mut Criterion) {
    c.bench_function("obsolete_tag_increment", |b| {
        b.iter(|| obsolete_tag_increment())
    });
}

fn bench_mining_worker(c: &mut Criterion) {
    c.bench_function("mining_worker", |b| b.iter(|| mining_worker()));
}

criterion_group!(
    benches,
    bench_tag_creation,
    bench_obsolete_tag_increment,
    bench_mining_worker
);
criterion_main!(benches);
