// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example bundle_miner --release
use iota::bundle_miner::MinerBuilder;
use iota::ternary::{T1B1Buf, TritBuf, TryteBuf};
#[tokio::main]
async fn main() {
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
    let security_level: usize = 3;
    let mut miner = MinerBuilder::new()
        .with_offset(0)
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
        .with_worker_count(1)
        .with_core_thread_count(1)
        .with_mining_timeout(40)
        .finish()
        .unwrap();
    let target_crack_probability = None;
    let threshold = None;
    // We can set extra parameters when running the bundle miner to ease of more customized usage.
    let miner_result = miner
        .run(target_crack_probability, threshold)
        .await
        .unwrap();
    println!("{:?}", miner_result);
}
