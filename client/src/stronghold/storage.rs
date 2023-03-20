// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! The `StorageProvider` implementation for `StrongholdAdapter`.

use std::ops::Deref;

use async_trait::async_trait;
use crypto::ciphers::chacha;

use super::{common::PRIVATE_DATA_CLIENT_PATH, StrongholdAdapter};
use crate::{storage::StorageProvider, Error, Result};

#[async_trait]
impl StorageProvider for StrongholdAdapter {
    #[allow(clippy::significant_drop_tightening)]
    async fn get(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>> {
        let data = match self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .get(k)?
        {
            Some(data) => data,
            None => return Ok(None),
        };

        let locked_key_provider = self.key_provider.lock().await;
        let key_provider = if let Some(key_provider) = &*locked_key_provider {
            key_provider
        } else {
            return Err(Error::StrongholdKeyCleared);
        };
        let buffer = key_provider.try_unlock()?;
        let buffer_ref = buffer.borrow();

        Ok(Some(chacha::aead_decrypt(buffer_ref.deref(), &data)?))
    }

    async fn insert(&mut self, k: &[u8], v: &[u8]) -> Result<Option<Vec<u8>>> {
        let encrypted_value = {
            let locked_key_provider = self.key_provider.lock().await;
            let key_provider = if let Some(key_provider) = &*locked_key_provider {
                key_provider
            } else {
                return Err(Error::StrongholdKeyCleared);
            };
            let buffer = key_provider.try_unlock()?;
            let buffer_ref = buffer.borrow();

            chacha::aead_encrypt(buffer_ref.deref(), v)?
        };

        Ok(self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .insert(k.to_vec(), encrypted_value, None)?)
    }

    async fn delete(&mut self, k: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self
            .stronghold
            .lock()
            .await
            .get_client(PRIVATE_DATA_CLIENT_PATH)?
            .store()
            .delete(k)?)
    }
}

mod tests {
    #[tokio::test]
    async fn test_stronghold_db() {
        use std::fs;

        use super::StrongholdAdapter;
        use crate::storage::StorageProvider;

        let snapshot_path = "test_stronghold_db.stronghold";
        let mut stronghold = StrongholdAdapter::builder()
            .password("drowssap")
            .build(snapshot_path)
            .unwrap();

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

        fs::remove_file(snapshot_path).unwrap();
    }
}
