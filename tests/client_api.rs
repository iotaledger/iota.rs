// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

use iota_client::Client;

const DEFAULT_NODE_POOL_URLS: &str = "http://localhost:14265";

#[ignore]
#[tokio::test]
async fn test_with_node_pool_urls() {
    let r = Client::builder()
        .with_node_pool_urls(&[DEFAULT_NODE_POOL_URLS.into()])
        .await
        .unwrap()
        .finish()
        .await;
    println!("{:#?}", r);
}
