use anyhow::Result;
use iota::bundle::TransactionField;
use iota::crypto::Kerl;
use iota::signing::{IotaSeed, Seed};
use iota::ternary::Trits;
use std::slice;

#[no_mangle]
pub extern "C" fn iota_get_new_address(seed: *const i8, index: u64, err: &mut u8) -> *const i8 {
    *err = 0;

    get_new_address(seed, index).unwrap_or_else(|_| {
        *err = 1;
        std::ptr::null_mut()
    })
}

fn get_new_address(seed: *const i8, index: u64) -> Result<*const i8> {
    let seed = unsafe {
        assert!(!seed.is_null());

        slice::from_raw_parts(seed, 243)
    };
    let seed =
        IotaSeed::<Kerl>::from_buf(Trits::try_from_raw(seed, 243).unwrap().to_owned()).unwrap();

    let (_, address) = smol::block_on(async move {
        iota::Client::get_new_address(&seed)
            .index(index)
            .generate()
            .await
    })?;
    // TODO Define a CAddress type
    let ptr = address.to_inner().as_i8_slice().as_ptr();
    std::mem::forget(address);

     Ok(ptr)
}
