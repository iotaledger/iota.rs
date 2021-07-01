// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use getset::{CopyGetters, Getters};
use serde::{Serialize, Deserialize};

use iota_client::crypto::keys::slip10::{
    Chain as RustChain,
};

/// Helper struct for offline signing
#[derive(Clone, Getters, CopyGetters, Serialize, Deserialize)]
pub struct Chain(Vec<u32>);

impl Chain {
    
    pub fn to_rust_chain(&self) -> RustChain {
        RustChain::from_u32(self.0)
    }
}

impl core::fmt::Display for Chain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "",
        )
    }
}

impl core::fmt::Debug for Chain {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Chain({})", self)
    }
}

impl From<RustChain> for Chain {
    fn from(chain: RustChain) -> Self {
        Self {
            
        }
    }
}