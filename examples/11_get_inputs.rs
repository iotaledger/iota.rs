// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 11_get_inputs --release

use iota_client::{Client, Result};
use bee_message::prelude::TransactionId;
use std::str::FromStr;

/// In this example, we retrieve the inputs of a transaction.

#[tokio::main]
async fn main() -> Result<()> {
    let iota = Client::builder()
        .with_node("https://api.lb-0.h.chrysalis-devnet.iota.cafe")?
        .finish()
        .await?;

    let transaction_id = TransactionId::from_str("131f4b296cf0e3283d6d8cfeb925bdbec4d5a42dcb58f5ef4b9f85b87f5e6f16").unwrap();
    let inputs = iota.get_inputs(&transaction_id).await?;
    for (idx, input) in inputs.iter().enumerate() {
        println!(
            "{}: {:#?}",
            idx,
            input,
        );
    }

    Ok(())
}
