//! Get transaction from a permanode.
//!
//! Run with:
//!
//! ```
//! cargo run --example permanode
//! ```
use anyhow::Result;
use iota::ternary::{TryteBuf};
use iota::transaction::bundled::{Address, BundledTransactionField};

#[smol_potat::main]
async fn main() -> Result<()> {
    let address = Address::from_inner_unchecked(
        TryteBuf::try_from_str(
            "LEYNSIMADMXAUYRGXKKEXPHDMZLRISZBSRZXUMCIKP9JQDOXSCIUGKYFFNPPVPGCHEJAWWSDHCKGOORPC",
        )
        .unwrap()
        .as_trits()
        .encode(),
    );

    let iota = iota::ClientBuilder::new()
        .node("https://iotanode.us:14267")?
        // .permanode("https://")?
        .build()?;

    let res = iota
        .find_transactions()
        .addresses(&[address])
        .send()
        .await?;

    println!("Res len: {:?}", res.hashes.len());

    Ok(())
}
