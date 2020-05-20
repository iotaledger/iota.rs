use anyhow::Result;
use crate::{CSeed, Transfers, Bundle};


#[no_mangle]
pub extern "C" fn iota_send_transfers(seed: *const CSeed, transfers: *mut Transfers, mwm: u8, err: &mut u8) -> *mut Bundle {
    *err = 0;

    send_transfers(seed, transfers, mwm).unwrap_or_else(|_| {
        *err = 1;
        std::ptr::null_mut()
    })
}

fn send_transfers(seed: *const CSeed, transfers: *mut Transfers, mwm: u8) -> Result<*mut Bundle> {
    let seed = unsafe {
        assert!(!seed.is_null());
        &(*seed).0
    };

    let transfers = unsafe {
        assert!(!transfers.is_null());
       (*Box::from_raw(transfers)).0
    };

    let res = smol::block_on(async move {
        iota::Client::send_transfers(&seed)
            .transfers(transfers)
            .min_weight_magnitude(mwm)
            .send()
            .await
    })?;

    Ok(Box::into_raw(Box::new(Bundle(res))))
}
