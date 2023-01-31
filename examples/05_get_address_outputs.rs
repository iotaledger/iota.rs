// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 05_get_address_outputs --release

use iota_client::{Client, Result};

/// In this example we will get the outputs of a known address

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")?
        .finish()
        .await?;

    let address = "atoi1qzt0nhsf38nh6rs4p6zs5knqp6psgha9wsv74uajqgjmwc75ugupx3y7x0r";

    let outputs = iota.get_address().outputs(address, Default::default()).await.unwrap();

    println!("The outputs of address {address:?} are: {outputs:?}");
    Ok(())
}
