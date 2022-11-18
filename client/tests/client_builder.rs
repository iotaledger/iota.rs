// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::{Client, ClientBuilder};

#[tokio::test]
async fn invalid_url() {
    let client = Client::builder().with_node("data:text/plain,Hello?World#");
    assert!(client.is_err());
}

#[tokio::test]
async fn valid_url() {
    let client = Client::builder().with_node("http://localhost:14265");
    assert!(client.is_ok());
}

#[tokio::test]
async fn client_builder() {
    let client_builder_json = r#"{
        "primaryNode":null,
        "primaryPowNode":null,
        "nodes":[
            {
                "url":"http://localhost:14265/",
                "auth":null,
                "disabled":false
            }
        ],
        "permanodes":null,
        "ignoreNodeHealth":true,
        "nodeSyncInterval":{
            "secs":60,
            "nanos":0
        },
        "quorum":false,
        "minQuorumSize":3,
        "quorumThreshold":66,
        "userAgent":"iota-client/2.0.1-rc.3",
        "protocolParameters":{
            "protocolVersion":2,
            "networkName":{
                "inner":"shimmer",
                "bounded":null
            },
            "bech32Hrp":{
                "inner":"smr",
                "bounded":null
            },
            "minPowScore":1500,
            "belowMaxDepth":15,
            "rentStructure":{
                "vByteCost":100,
                "vByteFactorKey":10,
                "vByteFactorData":1,
                "vByteOffset":380
            },
            "tokenSupply":1813620509061365
        },
        "localPow":true,
        "fallbackToLocalPow":true,
        "tipsInterval":5,
        "latestMilestoneTimestamp":null,
        "apiTimeout":{
            "secs":15,
            "nanos":0
        },
        "remotePowTimeout":{
            "secs":100,
            "nanos":0
        },
        "powWorkerCount":null
    }"#;

    let _client_builder = serde_json::from_str::<ClientBuilder>(client_builder_json).unwrap();

    // With protocol_version instead of protocolVersion
    let client_builder_json = r#"{
        "primaryNode":null,
        "primaryPowNode":null,
        "nodes":[
            {
                "url":"http://localhost:14265/",
                "auth":null,
                "disabled":false
            }
        ],
        "permanodes":null,
        "ignoreNodeHealth":true,
        "nodeSyncInterval":{
            "secs":60,
            "nanos":0
        },
        "quorum":false,
        "minQuorumSize":3,
        "quorumThreshold":66,
        "userAgent":"iota-client/2.0.1-rc.3",
        "protocolParameters":{
            "protocol_version":2,
            "networkName":{
                "inner":"shimmer",
                "bounded":null
            },
            "bech32Hrp":{
                "inner":"smr",
                "bounded":null
            },
            "minPowScore":1500,
            "belowMaxDepth":15,
            "rentStructure":{
                "vByteCost":100,
                "vByteFactorKey":10,
                "vByteFactorData":1,
                "vByteOffset":380
            },
            "tokenSupply":1813620509061365
        },
        "localPow":true,
        "fallbackToLocalPow":true,
        "tipsInterval":5,
        "latestMilestoneTimestamp":null,
        "apiTimeout":{
            "secs":15,
            "nanos":0
        },
        "remotePowTimeout":{
            "secs":100,
            "nanos":0
        },
        "powWorkerCount":null
    }"#;

    let _client_builder = serde_json::from_str::<ClientBuilder>(client_builder_json).unwrap();
}
