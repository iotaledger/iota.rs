//! Send a transfer to an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example send_transfers
//! ```
use anyhow::Result;
use iota::{
    transaction::bundled::{Address, BundledTransactionField},
    client::Transfer,
    crypto::ternary::Kerl,
    signing::ternary::{TernarySeed, Seed},
    ternary::{T1B1Buf, TryteBuf},
};
use iota_conversion::Trinary;

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
    iota::Client::add_node("https://nodes.comnet.thetangle.org")?;
    // Call send_transfers api
    // Below is just a dummy seed which just serves as an example.
    // If you want to replace your own. It probably should be a seed with balance on comnet/devnet.
    let res = iota::Client::send_transfers(Some(
        &TernarySeed::<Kerl>::from_trits(
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
    .transfers(transfers)
    // We are sending to comnet, so mwm should be 10. It's 14 by default if you don't call this.
    .min_weight_magnitude(10)
    // Sending to the node and receive the response
    .send()
    .await?;

    // The response of send_transfers is vector of Transaction type. We choose the first one and see what is its bundle hash
    println!("{:?}", res[0].bundle().to_inner().as_i8_slice().trytes());

    Ok(())
}
