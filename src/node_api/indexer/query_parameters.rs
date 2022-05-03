// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! Query parameters for output_id requests

use std::{
    collections::HashSet,
    hash::{Hash, Hasher},
    mem,
};

// https://github.com/gohornet/hornet/blob/bb1271be9f3a638f6acdeb6de74eab64515f27f1/plugins/indexer/v1/routes.go#L54

/// Query parameters for output_id requests
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryParameters(pub HashSet<QueryParameter>);

impl QueryParameters {
    /// Create a hashset from a provided vec of query parameters
    pub fn new(query_parameters: Vec<QueryParameter>) -> Self {
        let mut params = HashSet::new();
        for param in query_parameters {
            params.replace(param);
        }
        Self(params)
    }
    /// Replace or insert an enum variant in the HashSet
    pub fn replace(&mut self, query_parameter: QueryParameter) {
        self.0.replace(query_parameter);
    }
    /// Convert parameters to a single String
    pub fn into_query_sting(&self) -> Option<String> {
        if self.0.is_empty() {
            None
        } else {
            Some(
                self.0
                    .iter()
                    .map(|q| q.as_query_string())
                    .collect::<Vec<String>>()
                    .join("&"),
            )
        }
    }
}

/// Query parameter for output requests
#[derive(Debug, Clone, Serialize, Deserialize, Eq)]
pub enum QueryParameter {
    /// Bech32-encoded address that should be searched for.
    Address(String),
    /// Bech32-encoded alias address that should be searched for.
    AliasAddress(String),
    /// Filters outputs based on the presence of storage deposit return unlockcondition.
    HasStorageDepositReturnCondition(bool),
    /// Filter outputs based on the presence of a specific Bech32-encoded return address in the storage deposit return
    /// unlock condition.
    StorageDepositReturnAddress(String),
    /// Filters outputs based on the presence of timelock unlock condition.
    HasTimelockCondition(bool),
    /// Return outputs that are timelocked before a certain Unix timestamp.
    TimelockedBefore(u32),
    /// Return outputs that are timelocked after a certain Unix timestamp.
    TimelockedAfter(u32),
    /// Return outputs that are timelocked before a certain milestone index.
    TimelockedBeforeMilestone(u32),
    /// Return outputs that are timelocked ater a certain milestone index.
    TimelockedAfterMilestone(u32),
    /// Filters outputs based on the presence of expiration unlock condition.
    HasExpirationCondition(bool),
    /// Return outputs that expire before a certain Unix timestamp.
    ExpiresBefore(u32),
    /// Return outputs that expire after a certain Unix timestamp.
    ExpiresAfter(u32),
    /// Return outputs that expire before a certain milestone index.
    ExpiresBeforeMilestone(u32),
    /// Return outputs that expire after a certain milestone index.
    ExpiresAfterMilestone(u32),
    /// Filter outputs based on the presence of a specific Bech32-encoded return address in the expiration unlock
    /// condition.
    ExpirationReturnAddress(String),
    /// Filter for a certain sender
    Sender(String),
    /// Filter for a certain tags
    Tag(String),
    /// Return outputs that were created before a certain Unix timestamp.
    CreatedBefore(u32),
    /// Return outputs that were created after a certain Unix timestamp.
    CreatedAfter(u32),
    /// Pass the cursor(confirmationMS+outputId.pageSize) to start the results from
    Cursor(String),
    /// Filter for a certain issuer
    Issuer(String),
    /// Filter for a certain state controller address
    StateController(String),
    /// Filter for a certain governance controller address
    Governor(String),
    /// Define the page size for the results
    PageSize(usize),
}

// Custom impl because we only want a single query of each enum variant in the HashSet
impl PartialEq for QueryParameter {
    fn eq(&self, other: &Self) -> bool {
        mem::discriminant(self) == mem::discriminant(other)
    }
}
// Custom impl because we only want a single query of each enum variant in the HashSet
impl Hash for QueryParameter {
    fn hash<H: Hasher>(&self, state: &mut H) {
        0.hash(state);
    }
}

impl QueryParameter {
    fn as_query_string(&self) -> String {
        match self {
            QueryParameter::Address(v) => format!("address={}", v),
            QueryParameter::AliasAddress(v) => format!("aliasAddress={}", v),
            QueryParameter::HasStorageDepositReturnCondition(v) => format!("hasStorageDepositReturnCondition={}", v),
            QueryParameter::StorageDepositReturnAddress(v) => format!("storageReturnAddress={}", v),
            QueryParameter::HasTimelockCondition(v) => format!("hasTimelockCondition={}", v),
            QueryParameter::TimelockedBefore(v) => format!("timelockedBefore={}", v),
            QueryParameter::TimelockedAfter(v) => format!("timelockedAfter={}", v),
            QueryParameter::TimelockedBeforeMilestone(v) => format!("timelockedBeforeMilestone={}", v),
            QueryParameter::TimelockedAfterMilestone(v) => format!("timelockedAfterMilestone={}", v),
            QueryParameter::HasExpirationCondition(v) => format!("hasExpirationCondition={}", v),
            QueryParameter::ExpiresBefore(v) => format!("expiresBefore={}", v),
            QueryParameter::ExpiresAfter(v) => format!("expiresAfter={}", v),
            QueryParameter::ExpiresBeforeMilestone(v) => format!("expiresBeforeMilestone={}", v),
            QueryParameter::ExpiresAfterMilestone(v) => format!("expiresAfterMilestone={}", v),
            QueryParameter::ExpirationReturnAddress(v) => format!("expirationReturnAddress={}", v),
            QueryParameter::Sender(v) => format!("sender={}", v),
            QueryParameter::Tag(v) => format!("tag={}", v),
            QueryParameter::CreatedBefore(v) => format!("createdBefore={}", v),
            QueryParameter::CreatedAfter(v) => format!("createdAfter={}", v),
            QueryParameter::Cursor(v) => format!("cursor={}", v),
            QueryParameter::Issuer(v) => format!("issuer={}", v),
            QueryParameter::StateController(v) => format!("stateController={}", v),
            QueryParameter::Governor(v) => format!("governor={}", v),
            QueryParameter::PageSize(v) => format!("pageSize={}", v),
        }
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
        assert_eq!(address1, address2);
        assert_eq!(address1, address3);
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
