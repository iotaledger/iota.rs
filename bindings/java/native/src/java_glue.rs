// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, clippy::all)]

include!(concat!(env!("OUT_DIR"), "/java_glue.rs"));

// temp Fix for Sync trait; https://github.com/Dushistov/flapigen-rs/issues/388
unsafe impl Sync for JavaCallback {}
