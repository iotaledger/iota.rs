use crate::{Address, CSeed};
use anyhow::Result;

#[no_mangle]
pub extern "C" fn iota_get_new_address(
    seed: *const CSeed,
    index: u64,
    address: *mut Address,
) -> u8 {
    get_new_address(seed, index, address).unwrap_or_else(|_| 1)
}

fn get_new_address(seed: *const CSeed, index: u64, res: *mut Address) -> Result<u8> {
    let seed = unsafe {
        assert!(!seed.is_null());
        &(*seed).0
    };

    let res = unsafe {
        assert!(!res.is_null());
        &mut *res
    };

    let (_, address) = smol::block_on(async move {
        iota::Client::get_new_address(&seed)
            .index(index)
            .generate()
            .await
    })?;

    *res = Address(address);

    Ok(0)
}
