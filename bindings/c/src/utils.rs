use std::slice;

use iota::crypto::ternary::Kerl;
use iota::signing::ternary::{
    TernarySeed, PrivateKey, PrivateKeyGenerator, PublicKey, Seed, wots::WotsSpongePrivateKeyGeneratorBuilder,
    wots::WotsSecurityLevel,
};
use iota::ternary::Trits;

#[no_mangle]
pub extern "C" fn iota_address_gen(seed: *const i8, index: u64) -> *const i8 {
    let seed = unsafe {
        assert!(!seed.is_null());

        slice::from_raw_parts(seed, 243)
    };
    let seed =
        TernarySeed::<Kerl>::from_trits(Trits::try_from_raw(seed, 243).unwrap().to_owned()).unwrap();

    let address = WotsSpongePrivateKeyGeneratorBuilder::<Kerl>::default()
        .security_level(WotsSecurityLevel::Low)
        .build()
        .unwrap()
        .generate_from_seed(&seed, index)
        .unwrap()
        .generate_public_key()
        .unwrap();
    let ptr = address.to_trits().as_i8_slice().as_ptr();
    std::mem::forget(address);

    ptr
}
