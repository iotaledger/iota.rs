// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Constants for the Client

use std::time::Duration;

/// Default timeout for all API requests apart from post_message with remote PoW
pub(crate) const DEFAULT_API_TIMEOUT: Duration = Duration::from_secs(15);
pub(crate) const DEFAULT_REMOTE_POW_API_TIMEOUT: Duration = Duration::from_secs(50);
/// Interval in seconds when new tips will be requested during PoW, so the final message always will be attached to a
/// new part of the Tangle
pub(crate) const DEFAULT_TIPS_INTERVAL: u64 = 15;
pub(crate) const DEFAULT_MIN_POW: f64 = 4000f64;
pub(crate) const DEFAULT_BECH32_HRP: &str = "iota";
/// Interval in which the nodeinfo will be requested and healty nodes will be added to the synced node pool
pub(crate) const NODE_SYNC_INTERVAL: Duration = Duration::from_secs(60);
pub(crate) const DEFAULT_MIN_QUORUM_SIZE: usize = 3;
pub(crate) const DEFAULT_QUORUM_THRESHOLD: usize = 66;
#[cfg(not(feature = "wasm"))]
pub(crate) const MAX_PARALLEL_API_REQUESTS: usize = 100;
