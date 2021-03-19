//! Get an unused address from a connected node.
//!
//! Run with:
//!
//! ```
//! cargo run --example permanode
//! ```
use anyhow::Result;
use iota::ternary::{T1B1Buf, T3B1Buf, TryteBuf};
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

    // The response of get_new_address is a tuple of an adress with its corresponding index from seed.
    let iota = iota::ClientBuilder::new()
        .node("https://iotanode.us:14267")?
        .permanode("https://")?
        .build()?;

    let res = iota
        .find_transactions()
        .addresses(&[address])
        .send()
        .await?;

    println!("res len: {:?}", res.hashes.len());

    Ok(())
}
