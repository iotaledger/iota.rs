// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `DatabaseProvider` implementation for `StrongholdAdapter`.

use async_trait::async_trait;

use super::{
    common::PRIVATE_DATA_CLIENT_PATH,
    encryption::{decrypt, encrypt},
    StrongholdAdapter,
};
use crate::{db::DatabaseProvider, Error, Result};

#[async_trait]
impl DatabaseProvider for StrongholdAdapter {
    async fn get(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>> {
        // Lazy load the snapshot.
        self.read_stronghold_snapshot().await?;

        let data = match self
            .stronghold
            .lock()
            .await
            .load_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .get(k)?
        {
            Some(data) => data,
            None => return Ok(None),
        };

        let locked_key = self.key.lock().await;
        let key = if let Some(key) = &*locked_key {
            key
        } else {
            return Err(Error::StrongholdKeyCleared);
        };

        decrypt(&data, key).map(Some)
    }

    async fn insert(&mut self, k: &[u8], v: &[u8]) -> Result<Option<Vec<u8>>> {
        // Lazy load the snapshot.
        self.read_stronghold_snapshot().await?;

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

        self.stronghold
            .lock()
            .await
            .load_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .insert(k.to_vec(), encrypted_value, None)?;

        Ok(old_value)
    }

    async fn delete(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>> {
        // Lazy load the snapshot.
        self.read_stronghold_snapshot().await?;

        Ok(self
            .stronghold
            .lock()
            .await
            .load_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .delete(k)?)
    }
}

mod tests {
    #[tokio::test]
    async fn test_stronghold_db() {
        use std::path::PathBuf;

        use super::StrongholdAdapter;
        use crate::db::DatabaseProvider;

        let stronghold_path = PathBuf::from("test.stronghold");
        let mut stronghold = StrongholdAdapter::builder().password("drowssap").build(stronghold_path);

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
