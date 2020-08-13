//! Get an unused address from a connected node.
//!
//! Run with:
//!
//! ```
//! cargo run --example generate_new_address
//! ```
use anyhow::Result;
use iota::signing::ternary::seed::Seed;
use iota::ternary::{T1B1Buf, T3B1Buf, TryteBuf};
use iota::transaction::bundled::BundledTransactionField;

#[smol_potat::main]
async fn main() -> Result<()> {
    // Create seed from your seed trytes
    let seed = Seed::from_trits(
        TryteBuf::try_from_str(
            "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA",
        )
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>(),
    )
    .unwrap();

    // The response of get_new_address is a tuple of an adress with its corresponding index from seed.
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.comnet.thetangle.org")?
        .build()?;
    let (index, address) = iota.generate_new_address(&seed).generate().await.unwrap();

    println!(
        "Index: {}, Address:{:?}",
        index,
        address
            .to_inner()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
    );

    Ok(())
}
