// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `DatabaseProvider` implementation for `StrongholdAdapter`.

use async_trait::async_trait;
use iota_stronghold::{Location, ResultMessage};
use log::debug;

use super::{
    encryption::{decrypt, encrypt},
    StrongholdAdapter,
};
use crate::{db::DatabaseProvider, Error, Result};

/// Convert from a string to a Stronghold location that we'll use.
fn location_from_key(key: &[u8]) -> Location {
    // This has been the case in wallet.rs; we preserve it here.
    Location::Generic {
        vault_path: key.to_vec(),
        record_path: key.to_vec(),
    }
}

#[async_trait]
impl DatabaseProvider for StrongholdAdapter {
    async fn get(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>> {
        // Lazy load the snapshot (if the path is set).
        if self.snapshot_path.is_some() {
            self.read_stronghold_snapshot().await?;
        }

        let location = location_from_key(k);
        let (data, status) = self.stronghold.lock().await.read_from_store(location).await;

        // XXX: this theoretically indicates a non-existent key, but what about other errors?
        if let ResultMessage::Error(err) = status {
            debug!("Stronghold reported \"{}\", but we treat it as \"key not found\".", err);
            return Ok(None);
        }

        let locked_key = self.key.lock().await;
        let key = if let Some(key) = &*locked_key {
            key
        } else {
            return Err(Error::StrongholdKeyCleared);
        };

        decrypt(&data, key).map(Some)
    }

    async fn insert(&mut self, k: &[u8], v: &[u8]) -> Result<Option<Vec<u8>>> {
        // Lazy load the snapshot (if the path is set).
        if self.snapshot_path.is_some() {
            self.read_stronghold_snapshot().await?;
        }

        let old_value = self.get(k).await?;

        let encrypted_value = {
            let locked_key = self.key.lock().await;
            let key = if let Some(key) = &*locked_key {
                key
            } else {
                return Err(Error::StrongholdKeyCleared);
            };

            encrypt(v, key)?
        };

        let location = location_from_key(k);
        let status = self
            .stronghold
            .lock()
            .await
            .write_to_store(location, encrypted_value, None)
            .await;

        if let ResultMessage::Error(err) = status {
            return Err(Error::StrongholdProcedureError(err));
        }

        Ok(old_value)
    }

    async fn delete(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>> {
        // Lazy load the snapshot (if the path is set).
        if self.snapshot_path.is_some() {
            self.read_stronghold_snapshot().await?;
        }

        let old_value = self.get(k).await?;

        let location = location_from_key(k);
        let status = self.stronghold.lock().await.delete_from_store(location).await;

        if let ResultMessage::Error(err) = status {
            return Err(Error::StrongholdProcedureError(err));
        }

        Ok(old_value)
    }
}

mod tests {
    #[tokio::test]
    async fn test_stronghold_db() {
        use super::StrongholdAdapter;
        use crate::db::DatabaseProvider;

        let mut stronghold = StrongholdAdapter::builder().password("drowssap").build();

        assert!(matches!(stronghold.get(b"test-0").await, Ok(None)));
        assert!(matches!(stronghold.get(b"test-1").await, Ok(None)));
        assert!(matches!(stronghold.get(b"test-2").await, Ok(None)));

        assert!(matches!(stronghold.insert(b"test-0", b"test-0").await, Ok(None)));
        assert!(matches!(stronghold.insert(b"test-1", b"test-1").await, Ok(None)));
        assert!(matches!(stronghold.insert(b"test-2", b"test-2").await, Ok(None)));

        assert!(matches!(stronghold.get(b"test-0").await, Ok(Some(_))));
        assert!(matches!(stronghold.get(b"test-1").await, Ok(Some(_))));
        assert!(matches!(stronghold.get(b"test-2").await, Ok(Some(_))));

        assert!(matches!(stronghold.insert(b"test-0", b"0-tset").await, Ok(Some(_))));
        assert!(matches!(stronghold.insert(b"test-1", b"1-tset").await, Ok(Some(_))));
        assert!(matches!(stronghold.insert(b"test-2", b"2-tset").await, Ok(Some(_))));

        assert!(matches!(stronghold.delete(b"test-0").await, Ok(Some(_))));
        assert!(matches!(stronghold.delete(b"test-1").await, Ok(Some(_))));
        assert!(matches!(stronghold.delete(b"test-2").await, Ok(Some(_))));

        assert!(matches!(stronghold.get(b"test-0").await, Ok(None)));
        assert!(matches!(stronghold.get(b"test-1").await, Ok(None)));
        assert!(matches!(stronghold.get(b"test-2").await, Ok(None)));
    }
}
