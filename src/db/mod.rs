// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Database provider interfaces and implementations.

#[cfg(feature = "stronghold")]
mod stronghold;

#[cfg(feature = "stronghold")]
pub use self::stronghold::StrongholdDatabaseProvider;

use crate::Result;
use async_trait::async_trait;

/// The interface for database providers.
#[async_trait]
pub trait DatabaseProvider {
    /// Get a value out of the database.
    async fn get(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>>;

    /// Insert a value into the database.
    ///
    /// If there exists a record under the same key as `k`, it will be replaced by the new value (`v`) and returned.
    async fn insert(&mut self, k: &[u8], v: &[u8]) -> Result<Option<Vec<u8>>>;

    /// Delete a value from the database.
    ///
    /// The deleted value is returned.
    async fn delete(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>>;
}
