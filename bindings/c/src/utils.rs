use std::slice;

use iota::crypto::Kerl;
use iota::signing::{
    IotaSeed, PrivateKey, PrivateKeyGenerator, PublicKey, Seed, WotsPrivateKeyGeneratorBuilder,
    WotsSecurityLevel,
};
use iota::ternary::Trits;

#[no_mangle]
pub extern "C" fn iota_address_gen(seed: *const i8, index: u64) -> *const i8 {
    let seed = unsafe {
        assert!(!seed.is_null());

        slice::from_raw_parts(seed, 243)
    };
    let seed =
        IotaSeed::<Kerl>::from_buf(Trits::try_from_raw(seed, 243).unwrap().to_owned()).unwrap();

    let address = WotsPrivateKeyGeneratorBuilder::<Kerl>::default()
        .security_level(WotsSecurityLevel::Low)
        .build()
        .unwrap()
        .generate(&seed, index)
        .unwrap()
        .generate_public_key()
        .unwrap();
    let ptr = address.as_bytes().as_ptr();
    std::mem::forget(address);

    ptr
}
