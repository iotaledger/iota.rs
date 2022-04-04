// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example client_config --release

use iota_client::Client;

/// In this example we will create a client from a JSON config

#[tokio::main]
async fn main() {
    // Create a client instance
    let client = Client::builder()
        .from_json(
            r#"{
                "nodes":[
                   {
                      "url":"http://localhost:14265/",
                      "auth":null,
                      "disabled":false
                   },
                   {
                      "url":"https://chrysalis-nodes.iota.cafe/",
                      "auth":null,
                      "disabled":false
                   }
                ],
                "localPow":true,
                "apiTimeout":{
                   "secs":20,
                   "nanos":0
                }
             }"#,
        )
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = client.get_info().await.unwrap();
    println!("Node Info: {:?}", info);
}
