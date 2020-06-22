use crate::{Bundle, Hash};
use anyhow::Result;

#[no_mangle]
pub extern "C" fn iota_traverse_bundle(hash: *const Hash, bundle: *mut Bundle) -> u8 {
    traverse_bundle(hash, bundle).unwrap_or(1)
}

fn traverse_bundle(hash: *const Hash, bundle: *mut Bundle) -> Result<u8> {
    let hash = unsafe {
        assert!(!hash.is_null());
        &*hash
    };

    let bundle = unsafe {
        assert!(!bundle.is_null());
        &mut *bundle
    };

    let res = smol::block_on(async move { iota::Client::traverse_bundle(hash).await })?;

    *bundle = Bundle(res);

    Ok(0)
}
