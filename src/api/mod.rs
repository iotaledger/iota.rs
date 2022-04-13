// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! High level APIs

mod address;
mod consolidation;
mod message_builder;
mod types;

pub use self::{
    address::*,
    consolidation::*,
    message_builder::{pow::*, *},
    types::*,
};

const ADDRESS_GAP_RANGE: u32 = 20;
