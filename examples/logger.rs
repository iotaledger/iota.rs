// Copyright 2022 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example logger --release

use iota_client::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // Generates a client.log file with logs for debugging
    let logger_output_config = fern_logger::LoggerOutputConfigBuilder::new()
        .name("client.log")
        .target_exclusions(&["h2", "hyper", "rustls"])
        .level_filter(log::LevelFilter::Debug);
    let config = fern_logger::LoggerConfig::build()
        .with_output(logger_output_config)
        .finish();
    fern_logger::logger_init(config).unwrap();

    // Take the node URL from command line argument or use one from env as default.
    let node_url = std::env::args().nth(1).unwrap_or_else(|| {
        // This example uses dotenv, which is not safe for use in production.
        dotenv::dotenv().ok();
        std::env::var("NODE_URL").unwrap()
    });

    // Create a client with that node.
    let client = Client::builder().with_node(&node_url)?.finish()?;

    // Get node info.
    let info = client.get_info().await?;

    println!("{info:#?}");

    Ok(())
}
