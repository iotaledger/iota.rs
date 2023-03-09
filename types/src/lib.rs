// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Common types required by nodes and clients APIs like blocks, responses and DTOs.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(clippy::nursery, missing_docs, rust_2018_idioms, warnings)]
#![allow(
    clippy::redundant_pub_crate,
    clippy::module_name_repetitions,
    clippy::missing_const_for_fn,
    clippy::significant_drop_in_scrutinee
)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "block")]
pub mod block;
