// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Constants for the Client

use std::time::Duration;

/// Default timeout for all API requests apart from post_message with remote PoW
pub(crate) const DEFAULT_API_TIMEOUT: Duration = Duration::from_secs(15);
pub(crate) const DEFAULT_REMOTE_POW_API_TIMEOUT: Duration = Duration::from_secs(100);
/// Interval in seconds when new tips will be requested during PoW, so the final message always will be attached to a
/// new part of the Tangle
pub(crate) const DEFAULT_TIPS_INTERVAL: u64 = 15;
pub(crate) const DEFAULT_MIN_POW: f64 = 4000f64;
/// Interval in which the nodeinfo will be requested and healty nodes will be added to the synced node pool
pub(crate) const NODE_SYNC_INTERVAL: Duration = Duration::from_secs(60);
pub(crate) const DEFAULT_MIN_QUORUM_SIZE: usize = 3;
pub(crate) const DEFAULT_QUORUM_THRESHOLD: usize = 66;
#[cfg(not(feature = "wasm"))]
pub(crate) const MAX_PARALLEL_API_REQUESTS: usize = 100;

/// Bech32 hrp for the IOTA mainnet https://github.com/satoshilabs/slips/blob/master/slip-0173.md
pub const IOTA_BECH32_HRP: &str = "iota";
/// Bech32 hrp for the IOTA testnet https://github.com/satoshilabs/slips/blob/master/slip-0173.md
pub const IOTA_TESTNET_BECH32_HRP: &str = "atoi";
/// Bech32 hrp for the Shimmer mainnet https://github.com/satoshilabs/slips/blob/master/slip-0173.md
pub const SHIMMER_BECH32_HRP: &str = "smr";
/// Bech32 hrp for the IOTA testnet https://github.com/satoshilabs/slips/blob/master/slip-0173.md
pub const SHIMMER_TESTNET_BECH32_HRP: &str = "rms";

/// BIP-0044 defines a logical hierarchy for deterministic wallets
pub const HD_WALLET_TYPE: u32 = 44;
/// IOTA coin type https://github.com/satoshilabs/slips/blob/master/slip-0044.md
pub const IOTA_COIN_TYPE: u32 = 4218;
/// Shimmer coin type https://github.com/satoshilabs/slips/blob/master/slip-0044.md
pub const SHIMMER_COIN_TYPE: u32 = 4219;
