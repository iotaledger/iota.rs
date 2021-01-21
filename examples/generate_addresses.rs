//! Generate addresses from a seed.
//!
//! Run with:
//!
//! ```
//! cargo run --example generate_addresses
//! ```
use anyhow::Result;
use iota::signing::ternary::seed::Seed;
use iota::ternary::{T1B1Buf, T3B1Buf, TryteBuf};
use iota::transaction::bundled::BundledTransactionField;
use iota::AddressBuilder;

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

    let addresses = AddressBuilder::builder()
        .with_seed(&seed)
        .with_range(0..100)
        .finish()
        .unwrap();

    let address_strings: Vec<(u64, String)> = addresses
        .iter()
        .map(|(index, address)| {
            (
                *index,
                address
                    .to_inner()
                    .encode::<T3B1Buf>()
                    .iter_trytes()
                    .map(char::from)
                    .collect::<String>(),
            )
        })
        .collect();

    println!("{:#?}", address_strings);

    Ok(())
}
