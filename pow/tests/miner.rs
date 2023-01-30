// Copyright 2020-2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_pow::{
    miner::{get_miner, get_miner_num_workers, MinerBuilder, MinerCancel},
    score::PowScorer,
};
use iota_types::block::rand::bytes::rand_bytes;

#[test]
fn get_miner_score() {
    let miner = get_miner(4000);
    let mut bytes = rand_bytes(256);

    let nonce = miner(&bytes[0..248]).unwrap();
    bytes[248..].copy_from_slice(&nonce.to_le_bytes());

    assert!(PowScorer::new().score(&bytes) >= 4000f64);
}

#[test]
fn get_miner_num_workers_score() {
    let miner = get_miner_num_workers(4000, 4);
    let mut bytes = rand_bytes(256);

    let nonce = miner(&bytes[0..248]).unwrap();
    bytes[248..].copy_from_slice(&nonce.to_le_bytes());

    assert!(PowScorer::new().score(&bytes) >= 4000f64);
}

#[test]
fn miner_cancel() {
    let cancel = MinerCancel::new();
    let miner = MinerBuilder::new()
        .with_num_workers(4)
        .with_cancel(cancel.clone())
        .finish();
    // Hardcoded bytes instead of randomly generated bytes as there will always be a small probability that random bytes
    // will provide an easy to compute nonce in less than the second we allow to test the miner cancellation.
    let bytes = [
        67, 109, 231, 137, 43, 175, 54, 114, 170, 148, 6, 36, 239, 11, 127, 211, 21, 121, 176, 198, 41, 238, 140, 158,
        184, 230, 40, 174, 37, 119, 134, 21, 218, 50, 244, 44, 21, 138, 196, 122, 31, 226, 6, 109, 91, 217, 142, 176,
        153, 146, 118, 228, 184, 36, 67, 187, 33, 136, 61, 165, 116, 45, 126, 65, 157, 67, 89, 1, 69, 109, 83, 26, 148,
        111, 233, 213, 164, 250, 15, 84, 89, 69, 176, 102, 215, 142, 4, 250, 19, 41, 228, 93, 196, 209, 239, 101, 220,
        23, 127, 195, 147, 142, 125, 220, 233, 224, 150, 155, 130, 43, 202, 152, 194, 37, 245, 186, 255, 202, 204, 9,
        207, 204, 143, 201, 115, 242, 5, 13, 227, 41, 7, 62, 132, 231, 139, 3, 141, 128, 150, 206, 137, 240, 51, 58,
        239, 228, 218, 231, 222, 70, 196, 234, 200, 117, 211, 103, 127, 254, 179, 72, 98, 147, 250, 242, 125, 79, 184,
        161, 175, 78, 173, 237, 61, 93, 79, 121, 219, 156, 129, 162, 229, 248, 107, 193, 80, 185, 205, 70, 201, 75,
        162, 77, 135, 224, 252, 141, 124, 231, 21, 51, 151, 154, 123, 149, 27, 74, 87, 157, 191, 172, 152, 171, 57, 32,
        151, 51, 66, 231, 13, 143, 27, 59, 116, 224, 123, 245, 213, 65, 183, 189, 125, 154, 145, 175, 46, 76, 103, 194,
        152, 222, 102, 50, 8, 233, 160, 125, 153, 64, 91, 100, 234, 113, 108, 220, 171, 192,
    ];
    let now = std::time::Instant::now();
    let handle = std::thread::spawn(move || miner.nonce(&bytes[0..248], 100000));

    std::thread::sleep(std::time::Duration::from_secs(1));

    cancel.trigger();

    assert!(now.elapsed().as_secs() < 2);
    assert!(matches!(handle.join().unwrap(), None));
}
