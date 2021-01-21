//! Send a transfer to an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example transfer_all
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
    let seed = Seed::from_trits(
        TryteBuf::try_from_str(
            "TROLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOLOFELOLOLOLOLOLOLOLOAD",
        )
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>(),
    )
    .unwrap();
    // Create a client instance
    let iota = iota::ClientBuilder::new()
        .node("https://nodes.devnet.iota.org")?
        .build()?;
    // Get inputs
    let inputs = iota.get_all_inputs().with_seed(&seed).finish().await?;
    println!("{:?}", inputs);

    // Prepare a vector of transfers
    let mut transfers = Vec::new();

    // Push the transfer to vector.
    transfers.push(Transfer {
        // Address is 81 trytes.
        address: Address::from_inner_unchecked(
            TryteBuf::try_from_str(
                "I9HZLJSWABQNFGUZQUETRIUAERKZZXSPGRWXZWPMQDWLIMHSNCMDKIOEVQBKTDBCDNYDOHAYOVBYJYEEY",
            )
            .unwrap()
            .as_trits()
            .encode(),
        ),
        value: 0,
        message: None,
        tag: None,
    });

    let res = iota
        .send(Some(&seed))
        .with_transfers(transfers)
        .with_inputs(inputs.1)
        .with_min_weight_magnitude(14)
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
