// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Stronghold-as-a-Secret-Manager.

use crate::stronghold::StrongholdAdapter;

/// Secret manager that uses [`iota_stronghold`] as the backing storage.
///
/// This is just an alias to the all-in-one [StrongholdAdapter].
pub type StrongholdSecretManager = StrongholdAdapter;
