use crate::{Bundle, CSeed, Transfers};
use anyhow::Result;

#[no_mangle]
pub extern "C" fn iota_send_transfers(
    seed: *const CSeed,
    transfers: *mut Transfers,
    mwm: u8,
    bundle: *mut Bundle,
) -> u8 {
    send_transfers(seed, transfers, mwm, bundle).unwrap_or(1)
}

fn send_transfers(
    seed: *const CSeed,
    transfers: *mut Transfers,
    mwm: u8,
    res: *mut Bundle,
) -> Result<u8> {
    let seed = unsafe {
        assert!(!seed.is_null());
        &(*seed).0
    };

    let res = unsafe {
        assert!(!res.is_null());
        &mut *res
    };

    let transfers = unsafe {
        assert!(!transfers.is_null());
        (*Box::from_raw(transfers)).0
    };

    let tx = smol::block_on(async move {
        iota::Client::send_transfers(&seed)
            .transfers(transfers)
            .min_weight_magnitude(mwm)
            .send()
            .await
    })?;

    *res = Bundle(tx);

    Ok(0)
}
