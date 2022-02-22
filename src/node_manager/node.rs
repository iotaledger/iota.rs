// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use url::Url;

use std::hash::Hash;

/// Node authentication object.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct NodeAuth {
    /// JWT.
    pub jwt: Option<String>,
    /// Username and password.
    pub basic_auth_name_pwd: Option<(String, String)>,
}

/// Node definition.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node {
    /// Node url.
    pub url: Url,
    /// Node auth options.
    pub auth: Option<NodeAuth>,
    /// Whether the node is disabled or not.
    #[serde(default)]
    pub disabled: bool,
}

impl From<Url> for Node {
    fn from(url: Url) -> Self {
        Self {
            url,
            auth: None,
            disabled: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum NodeDto {
    Url(Url),
    Node(Node),
}

impl From<NodeDto> for Node {
    fn from(node: NodeDto) -> Self {
        match node {
            NodeDto::Url(url) => url.into(),
            NodeDto::Node(node) => node,
        }
    }
}

impl From<&NodeDto> for Node {
    fn from(node: &NodeDto) -> Self {
        match node {
            NodeDto::Url(url) => url.clone().into(),
            NodeDto::Node(node) => node.clone(),
        }
    }
}

/// JSON struct for NodeDetail from the node_pool_urls
#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct NodeDetail {
    /// Iota node url
    pub(crate) node: String,
    /// Network id
    pub(crate) network_id: String,
    /// Implementation name
    pub(crate) implementation: String,
    /// Enabled PoW
    pub(crate) pow: bool,
}
