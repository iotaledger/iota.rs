// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::Result;
use anyhow::anyhow;

use iota_client::bee_message::prelude::IndexationPayload as RustIndexationPayload;

#[derive(Eq, PartialEq, Clone)]
pub struct IndexationPayload(RustIndexationPayload);

impl IndexationPayload {
    pub fn to_inner(self) -> RustIndexationPayload {
        self.0
    }

    pub fn new(index: &[u8], data: &[u8]) -> Result<IndexationPayload> {
        let index = RustIndexationPayload::new(index, data);
        match index {
            Err(e) => Err(anyhow!(e.to_string())),
            Ok(i) => Ok(IndexationPayload(i)),
        }
    }

    pub fn new_from_string(index: &str, data: &str) -> Result<IndexationPayload> {
        let index = RustIndexationPayload::new(index.as_bytes(), data.as_bytes());
        match index {
            Err(e) => Err(anyhow!(e.to_string())),
            Ok(i) => Ok(IndexationPayload(i)),
        }
    }

    pub fn index(&self) -> &[u8] {
        self.0.index()
    }

    pub fn index_string(&self) -> Result<String> {
        Ok(std::str::from_utf8(self.0.index()).unwrap().to_string())
    }

    pub fn data(&self) -> &[u8] {
        self.0.data()
    }

    pub fn data_string(&self) -> Result<String> {
        Ok(std::str::from_utf8(self.0.data()).unwrap().to_string())
    }

    pub fn deserialize(serialised_data: &str) -> Result<IndexationPayload> {
        let res = serde_json::from_str(serialised_data);

        match res {
            Ok(s) => Ok(IndexationPayload(s)),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }

    pub fn serialize(&self) -> Result<String> {
        let res = serde_json::to_string(&self.0);

        match res {
            Ok(s) => Ok(s),
            Err(e) => Err(anyhow::anyhow!(e.to_string())),
        }
    }
}

impl core::fmt::Display for IndexationPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "payload={:?}", self.0)
    }
}

impl core::fmt::Debug for IndexationPayload {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "IndexationPayload({self})")
    }
}
