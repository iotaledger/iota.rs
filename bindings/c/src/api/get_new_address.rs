use anyhow::Result;
use crate::{CSeed, Address};

#[no_mangle]
pub extern "C" fn iota_get_new_address(seed: *const CSeed, index: u64, err: &mut u8) -> *const Address {
    *err = 0;

    get_new_address(seed, index).unwrap_or_else(|_| {
        *err = 1;
        std::ptr::null_mut()
    })
}

fn get_new_address(seed: *const CSeed, index: u64) -> Result<*const Address> {
    let seed = unsafe {
        assert!(!seed.is_null());
       &(*seed).0
    };

    let (_, address) = smol::block_on(async move {
        iota::Client::get_new_address(&seed)
            .index(index)
            .generate()
            .await
    })?;
    // TODO Define a CAddress type
    let ptr = Box::into_raw(Box::new(Address(address)));

     Ok(ptr)
}
