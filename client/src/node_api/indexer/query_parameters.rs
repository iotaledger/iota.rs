// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Query parameters for output_id requests

use std::fmt;

use crate::{Error, Result};

// https://github.com/gohornet/hornet/blob/bb1271be9f3a638f6acdeb6de74eab64515f27f1/plugins/indexer/v1/routes.go#L54

/// Query parameters for output_id requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParameters(Vec<QueryParameter>);

impl QueryParameters {
    /// Creates a hashset from a provided vec of query parameters.
    #[must_use]
    pub fn new(mut query_parameters: Vec<QueryParameter>) -> Self {
        query_parameters.sort_unstable_by_key(QueryParameter::kind);
        query_parameters.dedup_by_key(|qp| qp.kind());

        Self(query_parameters)
    }

    /// Creates new empty QueryParameters.
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Replaces or inserts an enum variant in the QueryParameters.
    pub fn replace(&mut self, query_parameter: QueryParameter) {
        match self
            .0
            .binary_search_by_key(&query_parameter.kind(), QueryParameter::kind)
        {
            Ok(pos) => self.0[pos] = query_parameter,
            Err(pos) => self.0.insert(pos, query_parameter),
        }
    }

    /// Returns true if the slice contains an element with the given kind.
    pub(crate) fn contains(&self, kind: u8) -> bool {
        self.0.iter().any(|q| q.kind() == kind)
    }

    /// Converts parameters to a single String.
    pub fn to_query_string(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(
                self.0
                    .iter()
                    .map(QueryParameter::to_query_string)
                    .collect::<Vec<String>>()
                    .join("&"),
            )
        }
    }
}

/// Query parameter for output requests.
#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum QueryParameter {
    /// Bech32-encoded address that should be searched for.
    Address(String),
    /// Filter foundry outputs based on bech32-encoded address of the controlling alias.
    AliasAddress(String),
    /// Returns outputs that were created after a certain Unix timestamp.
    CreatedAfter(u32),
    /// Returns outputs that were created before a certain Unix timestamp.
    CreatedBefore(u32),
    /// Starts the search from the cursor (confirmationMS+outputId.pageSize).
    Cursor(String),
    /// Filters outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    ExpirationReturnAddress(String),
    /// Returns outputs that expire after a certain Unix timestamp.
    ExpiresAfter(u32),
    /// Returns outputs that expire before a certain Unix timestamp.
    ExpiresBefore(u32),
    /// Filters outputs based on bech32-encoded governor (governance controller) address.
    Governor(String),
    /// Filters outputs based on the presence of expiration unlock condition.
    HasExpiration(bool),
    /// Filters outputs based on the presence of native tokens.
    HasNativeTokens(bool),
    /// Filters outputs based on the presence of storage deposit return unlock condition.
    HasStorageDepositReturn(bool),
    /// Filters outputs based on the presence of timelock unlock condition.
    HasTimelock(bool),
    /// Filters outputs based on bech32-encoded issuer address.
    Issuer(String),
    /// Filters outputs that have at most a certain number of distinct native tokens.
    MaxNativeTokenCount(u32),
    /// Filters outputs that have at least a certain number of distinct native tokens.
    MinNativeTokenCount(u32),
    /// The maximum amount of items returned in one call. If there are more items, a cursor to the next page is
    /// returned too. The parameter is ignored when pageSize is defined via the cursor parameter.
    PageSize(usize),
    /// Filters outputs based on the presence of validated Sender (bech32 encoded).
    Sender(String),
    /// Filters outputs based on bech32-encoded state controller address.
    StateController(String),
    /// Filters outputs based on the presence of a specific return address in the storage deposit return unlock
    /// condition.
    StorageDepositReturnAddress(String),
    /// Filters outputs based on matching Tag Block.
    Tag(String),
    /// Returns outputs that are timelocked after a certain Unix timestamp.
    TimelockedAfter(u32),
    /// Returns outputs that are timelocked before a certain Unix timestamp.
    TimelockedBefore(u32),
}

impl QueryParameter {
    fn to_query_string(&self) -> String {
        match self {
            Self::Address(v) => format!("address={v}"),
            Self::AliasAddress(v) => format!("aliasAddress={v}"),
            Self::CreatedAfter(v) => format!("createdAfter={v}"),
            Self::CreatedBefore(v) => format!("createdBefore={v}"),
            Self::Cursor(v) => format!("cursor={v}"),
            Self::ExpirationReturnAddress(v) => format!("expirationReturnAddress={v}"),
            Self::ExpiresAfter(v) => format!("expiresAfter={v}"),
            Self::ExpiresBefore(v) => format!("expiresBefore={v}"),
            Self::Governor(v) => format!("governor={v}"),
            Self::HasExpiration(v) => format!("hasExpiration={v}"),
            Self::HasNativeTokens(v) => format!("hasNativeTokens={v}"),
            Self::HasStorageDepositReturn(v) => format!("hasStorageDepositReturn={v}"),
            Self::HasTimelock(v) => format!("hasTimelock={v}"),
            Self::Issuer(v) => format!("issuer={v}"),
            Self::MaxNativeTokenCount(v) => format!("maxNativeTokenCount={v}"),
            Self::MinNativeTokenCount(v) => format!("minNativeTokenCount={v}"),
            Self::PageSize(v) => format!("pageSize={v}"),
            Self::Sender(v) => format!("sender={v}"),
            Self::StateController(v) => format!("stateController={v}"),
            Self::StorageDepositReturnAddress(v) => format!("storageDepositReturnAddress={v}"),
            Self::Tag(v) => format!("tag={v}"),
            Self::TimelockedAfter(v) => format!("timelockedAfter={v}"),
            Self::TimelockedBefore(v) => format!("timelockedBefore={v}"),
        }
    }

    pub(crate) fn kind(&self) -> u8 {
        match self {
            Self::Address(_) => 0,
            Self::AliasAddress(_) => 1,
            Self::CreatedAfter(_) => 2,
            Self::CreatedBefore(_) => 3,
            Self::Cursor(_) => 4,
            Self::ExpirationReturnAddress(_) => 5,
            Self::ExpiresAfter(_) => 6,
            Self::ExpiresBefore(_) => 7,
            Self::Governor(_) => 8,
            Self::HasExpiration(_) => 9,
            Self::HasNativeTokens(_) => 10,
            Self::HasStorageDepositReturn(_) => 11,
            Self::HasTimelock(_) => 12,
            Self::Issuer(_) => 13,
            Self::MaxNativeTokenCount(_) => 14,
            Self::MinNativeTokenCount(_) => 15,
            Self::PageSize(_) => 16,
            Self::Sender(_) => 17,
            Self::StateController(_) => 18,
            Self::StorageDepositReturnAddress(_) => 19,
            Self::Tag(_) => 20,
            Self::TimelockedAfter(_) => 21,
            Self::TimelockedBefore(_) => 22,
        }
    }
}

impl fmt::Display for QueryParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_query_string())
    }
}

macro_rules! verify_query_parameters {
    ($query_parameters:ident, $first:path $(, $rest:path)*) => {
        if let Some(qp) = $query_parameters.iter().find(|qp| {
            !matches!(qp, $first(_) $(| $rest(_))*)
        }) {
            Err(Error::UnsupportedQueryParameter(qp.clone()))
        } else {
            Ok(())
        }
    };
}

pub(crate) fn verify_query_parameters_basic_outputs(query_parameters: Vec<QueryParameter>) -> Result<QueryParameters> {
    verify_query_parameters!(
        query_parameters,
        QueryParameter::Address,
        QueryParameter::HasNativeTokens,
        QueryParameter::MinNativeTokenCount,
        QueryParameter::MaxNativeTokenCount,
        QueryParameter::HasStorageDepositReturn,
        QueryParameter::StorageDepositReturnAddress,
        QueryParameter::HasTimelock,
        QueryParameter::TimelockedBefore,
        QueryParameter::TimelockedAfter,
        QueryParameter::HasExpiration,
        QueryParameter::ExpiresBefore,
        QueryParameter::ExpiresAfter,
        QueryParameter::ExpirationReturnAddress,
        QueryParameter::Sender,
        QueryParameter::Tag,
        QueryParameter::CreatedBefore,
        QueryParameter::CreatedAfter,
        QueryParameter::PageSize,
        QueryParameter::Cursor
    )?;

    Ok(QueryParameters::new(query_parameters))
}

pub(crate) fn verify_query_parameters_alias_outputs(query_parameters: Vec<QueryParameter>) -> Result<QueryParameters> {
    verify_query_parameters!(
        query_parameters,
        QueryParameter::StateController,
        QueryParameter::Governor,
        QueryParameter::Issuer,
        QueryParameter::Sender,
        QueryParameter::HasNativeTokens,
        QueryParameter::MinNativeTokenCount,
        QueryParameter::MaxNativeTokenCount,
        QueryParameter::CreatedBefore,
        QueryParameter::CreatedAfter,
        QueryParameter::PageSize,
        QueryParameter::Cursor
    )?;

    Ok(QueryParameters::new(query_parameters))
}

pub(crate) fn verify_query_parameters_foundry_outputs(
    query_parameters: Vec<QueryParameter>,
) -> Result<QueryParameters> {
    verify_query_parameters!(
        query_parameters,
        QueryParameter::AliasAddress,
        QueryParameter::HasNativeTokens,
        QueryParameter::MinNativeTokenCount,
        QueryParameter::MaxNativeTokenCount,
        QueryParameter::CreatedBefore,
        QueryParameter::CreatedAfter,
        QueryParameter::PageSize,
        QueryParameter::Cursor
    )?;

    Ok(QueryParameters::new(query_parameters))
}

pub(crate) fn verify_query_parameters_nft_outputs(query_parameters: Vec<QueryParameter>) -> Result<QueryParameters> {
    verify_query_parameters!(
        query_parameters,
        QueryParameter::Address,
        QueryParameter::HasNativeTokens,
        QueryParameter::MinNativeTokenCount,
        QueryParameter::MaxNativeTokenCount,
        QueryParameter::HasStorageDepositReturn,
        QueryParameter::StorageDepositReturnAddress,
        QueryParameter::HasTimelock,
        QueryParameter::TimelockedBefore,
        QueryParameter::TimelockedAfter,
        QueryParameter::HasExpiration,
        QueryParameter::ExpiresBefore,
        QueryParameter::ExpiresAfter,
        QueryParameter::ExpirationReturnAddress,
        QueryParameter::Sender,
        QueryParameter::Tag,
        QueryParameter::CreatedBefore,
        QueryParameter::CreatedAfter,
        QueryParameter::PageSize,
        QueryParameter::Cursor
    )?;

    Ok(QueryParameters::new(query_parameters))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_parameter() {
        let address1 =
            QueryParameter::Address("atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".into());
        let address2 =
            QueryParameter::Address("atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".into());
        let address3 =
            QueryParameter::Address("atoi1qprxpfvaz2peggq6f8k9cj8zfsxuw69e4nszjyv5kuf8yt70t2847shpjak".into());
        let state_controller =
            QueryParameter::StateController("atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r".into());

        let mut query_parameters = QueryParameters::new(vec![address1, address2, state_controller]);
        // since address1 and address2 are of the same enum variant, we should only have one
        assert!(query_parameters.0.len() == 2);
        // since address2 and address3 are of the same enum variant, we should only have one
        query_parameters.replace(address3);
        assert!(query_parameters.0.len() == 2);
        // Contains address query parameter
        assert!(query_parameters.contains(QueryParameter::Address(String::new()).kind()));
        // Contains no cursor query parameter
        assert!(!query_parameters.contains(QueryParameter::Cursor(String::new()).kind()));
    }
}
