// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use crate::storage::{sqlite::SqliteStorageAdapter, StorageAdapter};
use bee_message::prelude::{Message, MessageId};
use bee_rest_api::types::MessageDto;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, ops::Deref, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

/// Message wrapper with message id
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageWrapper {
    id: String,
    message: MessageDto,
}

/// Account initialiser.
pub struct AccountInitialiser {
    id: String,
    storage_path: PathBuf,
    messages: Vec<MessageWrapper>,
    storage_adapter: Option<Box<dyn StorageAdapter + Send + Sync + 'static>>,
}

impl AccountInitialiser {
    /// Initialises the account builder.
    pub fn builder(id: String, storage_path: PathBuf) -> Self {
        Self {
            id,
            storage_path,
            messages: vec![],
            storage_adapter: None,
        }
    }
    /// Initialises the account builder.
    pub fn with_storage_adapter(mut self, storage_adapter: Box<dyn StorageAdapter + Send + Sync + 'static>) -> Self {
        self.storage_adapter = Some(storage_adapter);
        self
    }

    /// Messages associated with the seed.
    /// The account can be initialised with locally stored messages.
    pub fn with_messages(mut self, messages: Vec<Message>) -> Self {
        let messages_with_id: Vec<MessageWrapper> = messages
            .into_iter()
            .map(|msg| MessageWrapper {
                id: msg.id().0.to_string(),
                message: MessageDto::try_from(&msg).unwrap(),
            })
            .collect();
        self.messages = messages_with_id;
        self
    }

    /// Initialises the account.
    pub async fn finish(self) -> crate::Result<AccountHandle> {
        match self.storage_adapter {
            Some(adapter) => {
                crate::storage::set(&self.storage_path.clone(), adapter).await;
            }
            _ => {
                crate::storage::set(
                    &self.storage_path,
                    Box::new(SqliteStorageAdapter::new(&self.storage_path, &self.id)?),
                )
                .await;
            }
        }

        let mut account = Account {
            id: self.id,
            storage_path: self.storage_path.clone(),
            messages: self.messages,
        };

        account.save().await?;
        // let account_id = account.id().clone();
        let guard: AccountHandle = account.into();

        if let Some(parent) = self.storage_path.parent() {
            std::fs::create_dir_all(&parent)?;
        }

        Ok(guard)
    }
}

/// Account definition.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    /// The account identifier.
    id: String,
    /// Storage path
    storage_path: PathBuf,
    /// Messages sent
    /// The account can be initialised with locally stored messages.
    messages: Vec<MessageWrapper>,
}

/// A thread guard over an account.
#[derive(Debug, Clone)]
pub struct AccountHandle {
    inner: Arc<RwLock<Account>>,
}

impl From<Account> for AccountHandle {
    fn from(account: Account) -> Self {
        Self {
            inner: Arc::new(RwLock::new(account)),
        }
    }
}

impl Deref for AccountHandle {
    type Target = RwLock<Account>;
    fn deref(&self) -> &Self::Target {
        &self.inner.deref()
    }
}

impl AccountHandle {
    /// Bridge to [Account#list_messages](struct.Account.html#method.list_messages).
    /// This method clones the account's messages so when querying a large list of messages
    /// prefer using the `read` method to access the account instance.
    pub async fn list_messages(&self, count: usize, from: usize) -> Vec<MessageWrapper> {
        self.inner.read().await.list_messages(count, from).into_iter().collect()
    }
    /// Bridge to [Account#get_message](struct.Account.html#method.get_message).
    pub async fn get_message(&self, message_id: &MessageId) -> Option<MessageWrapper> {
        self.inner.read().await.get_message(message_id).cloned()
    }
}

impl Account {
    /// Set account id
    pub fn id(&self) -> String {
        self.id.clone()
    }

    /// Set storage path
    pub fn set_storage_path(&mut self, path: PathBuf) {
        self.storage_path = path;
    }

    pub(crate) async fn save(&mut self) -> crate::Result<()> {
        let storage_path = self.storage_path.clone();
        crate::storage::get(&storage_path)
            .await?
            .lock()
            .await
            .set(&self.id, serde_json::to_string(&self)?)
            .await?;
        Ok(())
    }

    /// Get Messages
    pub fn list_messages(&self, count: usize, from: usize) -> Vec<MessageWrapper> {
        let messages_iter = self.messages.clone().into_iter().skip(from);
        if count == 0 {
            messages_iter.collect()
        } else {
            messages_iter.take(count).collect()
        }
    }

    /// Add messages
    pub fn append_messages(&mut self, messages: Vec<Message>) {
        let messages_with_id: Vec<MessageWrapper> = messages
            .into_iter()
            .map(|msg| MessageWrapper {
                id: msg.id().0.to_string(),
                message: MessageDto::try_from(&msg).unwrap(),
            })
            .collect();
        self.messages.extend(messages_with_id);
    }

    /// Get mut messages
    pub fn messages_mut(&mut self) -> &mut Vec<MessageWrapper> {
        &mut self.messages
    }

    /// Gets a message with the given id associated with this account.
    pub fn get_message(&self, message_id: &MessageId) -> Option<&MessageWrapper> {
        self.messages.iter().find(|tx| tx.id == message_id.to_string())
    }
}
