// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Stronghold-as-a-Database implementation.

use crate::stronghold::StrongholdAdapter;

/// Stronghold as a database provider.
///
/// This is just an alias to the all-in-one [StrongholdAdapter].
pub type StrongholdStorageProvider = StrongholdAdapter;
