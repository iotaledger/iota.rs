// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Storage adapter

/// Account for storage
pub mod account;
/// Sqlite storage.
pub mod sqlite;
use account::Account;
pub use account::*;
use once_cell::sync::OnceCell;
use tokio::sync::{Mutex, RwLock};

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

/// Storage struct
pub struct Storage {
    storage_path: PathBuf,
    inner: Box<dyn StorageAdapter + Sync + Send>,
}

impl Storage {
    /// Get the id
    pub fn id(&self) -> &'static str {
        self.inner.id()
    }
    /// Get an account by id
    #[allow(dead_code)]
    pub async fn get(&mut self, account_id: &str) -> crate::Result<String> {
        self.inner.get(account_id).await
    }
    /// Get all accounts
    pub async fn get_all(&mut self) -> crate::Result<Vec<Account>> {
        parse_accounts(&self.storage_path, &self.inner.get_all().await?)
    }
    /// Set an account
    pub async fn set(&mut self, account_id: &str, account: String) -> crate::Result<()> {
        self.inner.set(account_id, account).await
    }
    /// Remove an account
    pub async fn remove(&mut self, account_id: &str) -> crate::Result<()> {
        self.inner.remove(account_id).await
    }
}

type StorageHandle = Arc<Mutex<Storage>>;
type Storages = Arc<RwLock<HashMap<PathBuf, StorageHandle>>>;
static INSTANCES: OnceCell<Storages> = OnceCell::new();

/// Sets the storage adapter.
pub async fn set<P: AsRef<Path>>(storage_path: P, storage: Box<dyn StorageAdapter + Send + Sync + 'static>) {
    let mut instances = INSTANCES.get_or_init(Default::default).write().await;
    instances.insert(
        storage_path.as_ref().to_path_buf(),
        Arc::new(Mutex::new(Storage {
            storage_path: storage_path.as_ref().to_path_buf(),
            inner: storage,
        })),
    );
}

/// gets the storage adapter
pub(crate) async fn get(storage_path: &PathBuf) -> crate::Result<StorageHandle> {
    let instances = INSTANCES.get_or_init(Default::default).read().await;
    if let Some(instance) = instances.get(storage_path) {
        Ok(instance.clone())
    } else {
        Err(crate::Error::StorageAdapterNotSet(
            storage_path.to_string_lossy().to_string(),
        ))
    }
}

/// The storage adapter.
#[async_trait::async_trait]
pub trait StorageAdapter {
    /// Gets the storage identifier (used internally on the default storage adapters)
    fn id(&self) -> &'static str {
        "custom-adapter"
    }
    /// Gets the account with the given id/alias from the storage.
    async fn get(&mut self, account_id: &str) -> crate::Result<String>;
    /// Gets all the accounts from the storage.
    async fn get_all(&mut self) -> crate::Result<Vec<String>>;
    /// Saves or updates an account on the storage.
    async fn set(&mut self, account_id: &str, account: String) -> crate::Result<()>;
    /// Removes an account from the storage.
    async fn remove(&mut self, account_id: &str) -> crate::Result<()>;
}

fn parse_accounts(storage_path: &PathBuf, accounts: &[String]) -> crate::Result<Vec<Account>> {
    let mut err = None;
    let accounts: Vec<Option<Account>> = accounts
        .iter()
        .map(|account| {
            if account.starts_with('{') {
                match serde_json::from_str::<Account>(&account.to_string()) {
                    Ok(mut acc) => {
                        acc.set_storage_path(storage_path.clone());
                        Some(acc)
                    }
                    Err(e) => {
                        err = Some(e.into());
                        None
                    }
                }
            } else {
                None
            }
        })
        .collect();

    if let Some(err) = err {
        Err(err)
    } else {
        let accounts = accounts.into_iter().map(|account| account.unwrap()).collect();
        Ok(accounts)
    }
}

#[cfg(test)]
mod tests {
    use super::StorageAdapter;

    #[tokio::test]
    // asserts that the adapter defined by `set` is globally available with `get`
    async fn set_adapter() {
        struct MyAdapter;
        #[async_trait::async_trait]
        impl StorageAdapter for MyAdapter {
            async fn get(&mut self, _key: &str) -> crate::Result<String> {
                Ok("MY_ADAPTER_GET_RESPONSE".to_string())
            }
            async fn get_all(&mut self) -> crate::Result<Vec<String>> {
                Ok(vec![])
            }
            async fn set(&mut self, _key: &str, _account: String) -> crate::Result<()> {
                Ok(())
            }
            async fn remove(&mut self, _key: &str) -> crate::Result<()> {
                Ok(())
            }
        }

        let path = "./the-storage-path";
        super::set(path, Box::new(MyAdapter {})).await;
        let adapter = super::get(&std::path::PathBuf::from(path)).await.unwrap();
        let mut adapter = adapter.lock().await;
        assert_eq!(adapter.get("").await.unwrap(), "MY_ADAPTER_GET_RESPONSE".to_string());
    }
}
