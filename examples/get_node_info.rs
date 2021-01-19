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
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.comnet.thetangle.org")?
        .build()?;
    let node_info = iota
        .get_node_info("https://nodes.comnet.thetangle.org")
        .await?;
    println!("{:#?}", node_info);
    Ok(())
}
