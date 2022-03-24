// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::ClientMiner as RustClientMiner;

pub struct ClientMiner(RustClientMiner);

impl ClientMiner {
    pub fn to_inner(self) -> RustClientMiner {
        self.0
    }
}

impl From<RustClientMiner> for ClientMiner {
    fn from(miner: RustClientMiner) -> Self {
        Self(miner)
    }
}
