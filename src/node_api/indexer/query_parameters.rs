// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Query parameters for output_id requests

use std::fmt;

// https://github.com/gohornet/hornet/blob/bb1271be9f3a638f6acdeb6de74eab64515f27f1/plugins/indexer/v1/routes.go#L54

/// Query parameters for output_id requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParameters(Vec<QueryParameter>);

impl QueryParameters {
    /// Creates a hashset from a provided vec of query parameters.
    pub fn new(mut query_parameters: Vec<QueryParameter>) -> Self {
        query_parameters.sort_unstable_by_key(|qp| qp.kind());
        query_parameters.dedup_by_key(|qp| qp.kind());

        Self(query_parameters)
    }

    /// Replaces or inserts an enum variant in the QueryParameters.
    pub fn replace(&mut self, query_parameter: QueryParameter) {
        match self.0.binary_search_by_key(&query_parameter.kind(), |qp| qp.kind()) {
            Ok(pos) => self.0[pos] = query_parameter,
            Err(pos) => self.0.insert(pos, query_parameter),
        }
    }

    /// Converts parameters to a single String.
    pub fn to_query_string(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(
                self.0
                    .iter()
                    .map(|q| q.to_query_string())
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
    /// Returns outputs that expire after a certain milestone index.
    ExpiresAfterMilestone(u32),
    /// Returns outputs that expire before a certain Unix timestamp.
    ExpiresBefore(u32),
    /// Returns outputs that expire before a certain milestone index.
    ExpiresBeforeMilestone(u32),
    /// Filters outputs based on bech32-encoded governor (governance controller) address.
    Governor(String),
    /// Filters outputs based on the presence of expiration unlock condition.
    HasExpirationCondition(bool),
    /// Filters outputs based on the presence of native tokens.
    HasNativeTokens(bool),
    /// Filters outputs based on the presence of storage return unlock condition.
    HasStorageReturnCondition(bool),
    /// Filters outputs based on the presence of timelock unlock condition.
    HasTimelockCondition(bool),
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
    /// Filters outputs based on the presence of a specific return address in the storage return unlock condition.
    StorageReturnAddress(String),
    /// Filters outputs based on matching Tag Block.
    Tag(String),
    /// Returns outputs that are timelocked after a certain Unix timestamp.
    TimelockedAfter(u32),
    /// Returns outputs that are timelocked ater a certain milestone index.
    TimelockedAfterMilestone(u32),
    /// Returns outputs that are timelocked before a certain Unix timestamp.
    TimelockedBefore(u32),
    /// Returns outputs that are timelocked before a certain milestone index.
    TimelockedBeforeMilestone(u32),
}

impl QueryParameter {
    fn to_query_string(&self) -> String {
        match self {
            QueryParameter::Address(v) => format!("address={}", v),
            QueryParameter::AliasAddress(v) => format!("aliasAddress={}", v),
            QueryParameter::CreatedAfter(v) => format!("createdAfter={}", v),
            QueryParameter::CreatedBefore(v) => format!("createdBefore={}", v),
            QueryParameter::Cursor(v) => format!("cursor={}", v),
            QueryParameter::ExpirationReturnAddress(v) => format!("expirationReturnAddress={}", v),
            QueryParameter::ExpiresAfter(v) => format!("expiresAfter={}", v),
            QueryParameter::ExpiresAfterMilestone(v) => format!("expiresAfterMilestone={}", v),
            QueryParameter::ExpiresBefore(v) => format!("expiresBefore={}", v),
            QueryParameter::ExpiresBeforeMilestone(v) => format!("expiresBeforeMilestone={}", v),
            QueryParameter::Governor(v) => format!("governor={}", v),
            QueryParameter::HasExpirationCondition(v) => format!("hasExpirationCondition={}", v),
            QueryParameter::HasNativeTokens(v) => format!("hasNativeTokens={}", v),
            QueryParameter::HasStorageReturnCondition(v) => format!("hasStorageReturnCondition={}", v),
            QueryParameter::HasTimelockCondition(v) => format!("hasTimelockCondition={}", v),
            QueryParameter::Issuer(v) => format!("issuer={}", v),
            QueryParameter::MaxNativeTokenCount(v) => format!("maxNativeTokenCount={}", v),
            QueryParameter::MinNativeTokenCount(v) => format!("minNativeTokenCount={}", v),
            QueryParameter::PageSize(v) => format!("pageSize={}", v),
            QueryParameter::Sender(v) => format!("sender={}", v),
            QueryParameter::StateController(v) => format!("stateController={}", v),
            QueryParameter::StorageReturnAddress(v) => format!("storageReturnAddress={}", v),
            QueryParameter::Tag(v) => format!("tag={}", v),
            QueryParameter::TimelockedAfter(v) => format!("timelockedAfter={}", v),
            QueryParameter::TimelockedAfterMilestone(v) => format!("timelockedAfterMilestone={}", v),
            QueryParameter::TimelockedBefore(v) => format!("timelockedBefore={}", v),
            QueryParameter::TimelockedBeforeMilestone(v) => format!("timelockedBeforeMilestone={}", v),
        }
    }

    fn kind(&self) -> u8 {
        match self {
            QueryParameter::Address(_) => 0,
            QueryParameter::AliasAddress(_) => 1,
            QueryParameter::CreatedAfter(_) => 2,
            QueryParameter::CreatedBefore(_) => 3,
            QueryParameter::Cursor(_) => 4,
            QueryParameter::ExpirationReturnAddress(_) => 5,
            QueryParameter::ExpiresAfter(_) => 6,
            QueryParameter::ExpiresAfterMilestone(_) => 7,
            QueryParameter::ExpiresBefore(_) => 8,
            QueryParameter::ExpiresBeforeMilestone(_) => 9,
            QueryParameter::Governor(_) => 10,
            QueryParameter::HasExpirationCondition(_) => 11,
            QueryParameter::HasNativeTokens(_) => 12,
            QueryParameter::HasStorageReturnCondition(_) => 13,
            QueryParameter::HasTimelockCondition(_) => 14,
            QueryParameter::Issuer(_) => 15,
            QueryParameter::MaxNativeTokenCount(_) => 16,
            QueryParameter::MinNativeTokenCount(_) => 17,
            QueryParameter::PageSize(_) => 18,
            QueryParameter::Sender(_) => 19,
            QueryParameter::StateController(_) => 20,
            QueryParameter::StorageReturnAddress(_) => 21,
            QueryParameter::Tag(_) => 22,
            QueryParameter::TimelockedAfter(_) => 23,
            QueryParameter::TimelockedAfterMilestone(_) => 24,
            QueryParameter::TimelockedBefore(_) => 25,
            QueryParameter::TimelockedBeforeMilestone(_) => 26,
        }
    }
}

impl fmt::Display for QueryParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_query_string())
    }
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
    }
}
