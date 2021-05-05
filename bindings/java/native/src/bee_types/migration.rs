// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0
/*
use getset::{CopyGetters, Getters};
use iota_wallet::{
    account_manager::{MigrationBundle as RustMigrationBundle, MigrationData as RustMigrationData},
    iota_migration::{
        client::response::InputData as RustInputData, ternary::T3B1Buf, transaction::bundled::BundledTransactionField,
    },
};
use std::convert::{From, Into};

#[derive(Debug, Getters, CopyGetters)]
pub struct InputData {
    #[getset(get = "pub")]
    address: String,
    #[getset(get_copy = "pub")]
    security_lvl: u8,
    #[getset(get_copy = "pub")]
    balance: u64,
    #[getset(get_copy = "pub")]
    index: u64,
    #[getset(get_copy = "pub")]
    spent: bool,
    #[getset(get = "pub")]
    spent_bundlehashes: Option<Vec<String>>,
}

impl From<RustInputData> for InputData {
    fn from(input: RustInputData) -> Self {
        Self {
            address: input
                .address
                .to_inner()
                .encode::<T3B1Buf>()
                .iter_trytes()
                .map(char::from)
                .collect::<String>(),
            security_lvl: input.security_lvl,
            balance: input.balance,
            index: input.index,
            spent: input.spent,
            spent_bundlehashes: input.spent_bundlehashes,
        }
    }
}

#[derive(Debug, Getters, CopyGetters)]
pub struct MigrationData {
    #[getset(get_copy = "pub")]
    balance: u64,
    #[getset(get_copy = "pub")]
    last_checked_address_index: u64,
    #[getset(get = "pub")]
    inputs: Vec<InputData>,
}

impl From<RustMigrationData> for MigrationData {
    fn from(migration_data: RustMigrationData) -> Self {
        Self {
            balance: migration_data.balance,
            last_checked_address_index: migration_data.last_checked_address_index,
            inputs: migration_data.inputs.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Getters, CopyGetters)]
pub struct MigrationBundle {
    #[getset(get_copy = "pub")]
    crackability: f64,
    #[getset(get = "pub")]
    bundle_hash: String,
}

impl From<RustMigrationBundle> for MigrationBundle {
    fn from(migration_bundle: RustMigrationBundle) -> Self {
        Self {
            crackability: *migration_bundle.crackability(),
            bundle_hash: migration_bundle.bundle_hash().clone(),
        }
    }
}
*/