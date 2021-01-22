//! Send a transfer to an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example send_transfers
//! ```
use anyhow::Result;
use iota::{
    client::Transfer,
    signing::ternary::seed::Seed,
    ternary::{T1B1Buf, T3B1Buf, TryteBuf},
    transaction::bundled::{Address, BundledTransactionField},
};

#[smol_potat::main]
async fn main() -> Result<()> {
    // Prepare a vector of transfers
    let mut transfers = Vec::new();

    // Push the transfer to vector.
    transfers.push(Transfer {
        // Address is 81 trytes.
        address: Address::from_inner_unchecked(
            TryteBuf::try_from_str(
                "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA",
            )
            .unwrap()
            .as_trits()
            .encode(),
        ),
        // We are using a zero balance seed so we make a zero value transfer here
        value: 0,
        message: None,
        tag: None,
    });

    // Create a client instance
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.comnet.thetangle.org")?
        .build()?;
    // Call send_transfers api
    // Below is just a dummy seed which just serves as an example.
    // If you want to replace your own. It probably should be a seed with balance on comnet/devnet.
    let res = iota.send(Some(
        &Seed::from_trits(
            TryteBuf::try_from_str(
                "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA",
            )
            .unwrap()
            .as_trits()
            .encode::<T1B1Buf>(),
        )
        .unwrap(),
    ))
    // Input the transfers
    .with_transfers(transfers)
    // We are sending to comnet, so mwm should be 10. It's 14 by default if you don't call this.
    .with_min_weight_magnitude(10)
    // Sending to the node and receive the response
    .finish()
    .await?;

    // The response of send_transfers is vector of Transaction type. We choose the first one and see what is its bundle hash
    println!(
        "{:?}",
        res[0]
            .bundle()
            .to_inner()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
    );

    Ok(())
}
