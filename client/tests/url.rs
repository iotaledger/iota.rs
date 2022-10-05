// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

#[tokio::test]
async fn invalid_url() {
    let client = iota_client::Client::builder().with_node("data:text/plain,Hello?World#");
    assert!(client.is_err());
}
#[tokio::test]
async fn valid_url() {
    let client = iota_client::Client::builder().with_node("http://localhost:14265");
    assert!(client.is_ok());
}
