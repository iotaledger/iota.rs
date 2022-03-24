// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// deref_nullptr: supressing UB warnings in bindgen generated code. See:
// https://github.com/rust-lang/rust-bindgen/issues/1651
#![allow(
    non_upper_case_globals,
    dead_code,
    non_camel_case_types,
    improper_ctypes,
    non_snake_case,
    deref_nullptr,
    clippy::unreadable_literal,
    clippy::redundant_static_lifetimes
)]

include!(concat!(env!("OUT_DIR"), "/jni_c_headers.rs"));
