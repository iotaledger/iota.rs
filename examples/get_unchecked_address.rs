//! Get an unchecked from seed. This is just for developing purpose in case you want to see certain address.
//! For general caes, users should use `get_new_address`.
//!
//! Run with:
//!
//! ```
//! cargo run --example get-unchecked-address
//! ```
use anyhow::Result;
use iota::bundle::{Address, TransactionField};
use iota::crypto::Kerl;
use iota::signing::{
    IotaSeed, PrivateKey, PrivateKeyGenerator, PublicKey, Seed, WotsPrivateKeyGeneratorBuilder,
    WotsSecurityLevel,
};
use iota::ternary::{T1B1Buf, TryteBuf};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    let seed = IotaSeed::<Kerl>::from_buf(
        TryteBuf::try_from_str(
            "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA",
        )
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>(),
    )
    .unwrap();

    let address: Address = Address::try_from_inner(
        WotsPrivateKeyGeneratorBuilder::<Kerl>::default()
            .security_level(WotsSecurityLevel::Medium)
            .build()
            .unwrap()
            .generate(&seed, 3)
            .unwrap()
            .generate_public_key()
            .unwrap()
            .trits()
            .to_owned(),
    )
    .unwrap();

    println!("Address:{:?}", address.to_inner().as_i8_slice().trytes());

    Ok(())
}
