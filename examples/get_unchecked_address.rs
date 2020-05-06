//! Get an address from seed and index that the address could be an used address on the Tangle.
//! This is just for developing purpose in case you want to see a certain address.
//!
//! Run with:
//!
//! ```
//! cargo run --example get_unchecked_address
//! ```
use iota::bundle::{Address, TransactionField};
use iota::crypto::Kerl;
use iota::signing::{
    IotaSeed, PrivateKey, PrivateKeyGenerator, PublicKey, Seed, WotsPrivateKeyGeneratorBuilder,
    WotsSecurityLevel,
};
use iota::ternary::{T1B1Buf, TryteBuf};
use iota_conversion::Trinary;

fn main() {
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
}
