// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Contains traits and implementations to provide and score Proof of Work.
//! TIP <https://github.com/iotaledger/tips/blob/main/tips/TIP-0012/tip-0012.md>.

#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![warn(missing_docs)]

mod error;
pub mod miner;
pub mod score;
#[cfg(target_family = "wasm")]
pub mod wasm_miner;

pub use error::Error;

// Precomputed natural logarithm of 3 for performance reasons.
// See https://oeis.org/A002391.
const LN_3: f64 = 1.098_612_288_668_109;
