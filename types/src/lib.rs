// Copyright 2020-2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Common types required by nodes and clients APIs like blocks, responses and DTOs.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(doc_cfg, feature(doc_cfg))]
#![deny(missing_docs, warnings)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "api")]
pub mod api;
#[cfg(feature = "block")]
pub mod block;
