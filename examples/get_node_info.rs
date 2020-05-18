//! Get node information from an IOTA node.
//!
//! Run with:
//!
//! ```
//! cargo run --example get_node_info
//! ```
use anyhow::Result;

#[smol_potat::main]
async fn main() -> Result<()> {
    iota::Client::add_node("https://nodes.comnet.thetangle.org")?;
    let node_info = iota::Client::get_node_info().await?;
    println!("{:#?}", node_info);
    Ok(())
}
