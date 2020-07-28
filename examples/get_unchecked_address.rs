//! Get an address from seed and index that the address could be an used address on the Tangle.
//! This is just for developing purpose in case you want to see a certain address.
//!
//! Run with:
//!
//! ```
//! cargo run --example get_unchecked_address
//! ```
use iota::crypto::ternary::sponge::Kerl;
use iota::signing::ternary::{
    seed::Seed,
    wots::{WotsSecurityLevel, WotsSpongePrivateKeyGeneratorBuilder},
    PrivateKey, PrivateKeyGenerator, PublicKey,
};
use iota::ternary::{T1B1Buf, T3B1Buf, TryteBuf};
use iota::transaction::bundled::{Address, BundledTransactionField};

fn main() {
    let seed = Seed::from_trits(
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
            .as_trits()
            .to_owned(),
    )
    .unwrap();

    println!(
        "Address:{:?}",
        address
            .to_inner()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
    );
}
