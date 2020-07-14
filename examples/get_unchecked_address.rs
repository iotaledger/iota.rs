//! Get an address from seed and index that the address could be an used address on the Tangle.
//! This is just for developing purpose in case you want to see a certain address.
//!
//! Run with:
//!
//! ```
//! cargo run --example get_unchecked_address
//! ```
use iota::transaction::bundled::{Address, BundledTransactionField};
use iota::crypto::ternary::Kerl;
use iota::signing::ternary::{
    TernarySeed, PrivateKey, PrivateKeyGenerator, PublicKey, Seed,
    wots::{
        WotsSpongePrivateKeyGeneratorBuilder,
        WotsSecurityLevel,
    },
};
use iota::ternary::{T1B1Buf, TryteBuf};
use iota_conversion::Trinary;

fn main() {
    let seed = TernarySeed::<Kerl>::from_buf(
        TryteBuf::try_from_str(
            "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA",
        )
        .unwrap()
        .as_trits()
        .encode::<T1B1Buf>(),
    )
    .unwrap();

    let address: Address = Address::try_from_inner(
        WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default()
            .security_level(WotsSecurityLevel::Medium)
            .build()
            .unwrap()
            .generate_from_seed(&seed, 3)
            .unwrap()
            .generate_public_key()
            .unwrap()
            .to_trits()
            .to_owned(),
    )
    .unwrap();

    println!("Address:{:?}", address.to_inner().as_i8_slice().trytes());
}
