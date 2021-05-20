// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
use iota_client::Seed as RustSeed;

pub struct Seed {
    seed: RustSeed,
}

impl Seed {
    pub fn to_inner(&self) -> &RustSeed {
        &self.seed
    }
}
