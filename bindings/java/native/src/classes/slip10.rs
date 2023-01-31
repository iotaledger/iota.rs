// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};

use iota_client::crypto::keys::slip10::Chain as RustChain;

/// Helper struct for offline signing
#[derive(Clone, Serialize, Deserialize)]
pub struct Chain(String);

impl Chain {
    pub fn to_rust_chain(&self) -> RustChain {
        serde_json::from_str(&self.0).unwrap()
    }
}

impl core::fmt::Display for Chain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.0,)
    }
}

impl core::fmt::Debug for Chain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Chain({self})")
    }
}

impl From<RustChain> for Chain {
    fn from(chain: RustChain) -> Self {
        Self(serde_json::to_string(&chain).unwrap())
    }
}
