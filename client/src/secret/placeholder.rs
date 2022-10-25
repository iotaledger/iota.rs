// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Implementation of [`PlaceholderSecretManager`].

use std::ops::Range;

use async_trait::async_trait;
use iota_types::block::{
    address::Address,
    unlock::{Unlock, Unlocks},
};

use super::{types::InputSigningData, GenerateAddressOptions, SecretManage, SecretManageExt};
use crate::secret::{PreparedTransactionData, RemainderData};

/// Secret manager that is only useful to prevent accidental address generation in a wallet
/// that has an offline counterpart for address generation and signing.
pub struct PlaceholderSecretManager;

#[async_trait]
impl SecretManage for PlaceholderSecretManager {
    async fn generate_addresses(
        &self,
        _coin_type: u32,
        _account_index: u32,
        _address_indexes: Range<u32>,
        _internal: bool,
        _: Option<GenerateAddressOptions>,
    ) -> crate::Result<Vec<Address>> {
        return Err(crate::Error::PlaceholderSecretManager);
    }

    async fn signature_unlock(
        &self,
        _input: &InputSigningData,
        _essence_hash: &[u8; 32],
        _: &Option<RemainderData>,
    ) -> crate::Result<Unlock> {
        return Err(crate::Error::PlaceholderSecretManager);
    }
}

#[async_trait]
impl SecretManageExt for PlaceholderSecretManager {
    async fn sign_transaction_essence(
        &self,
        _prepared_transaction_data: &PreparedTransactionData,
    ) -> crate::Result<Unlocks> {
        return Err(crate::Error::PlaceholderSecretManager);
    }
}
