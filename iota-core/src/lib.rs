// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! IOTA core

#![deny(unused_extern_crates)]
#![warn(
    //missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]

pub use crate::bee_message as message;
pub use iota_client as client;

pub use client::{Client, ClientBuilder};

// TODO prelude
pub use client::*;
pub use message::prelude;
pub use prelude::*;
