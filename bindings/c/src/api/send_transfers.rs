use anyhow::Result;
use crate::{CSeed, Transfers};


#[no_mangle]
pub extern "C" fn iota_send_transfers(seed: *const CSeed, transfers: *mut Transfers, mwm: u8, err: &mut u8) {
    *err = 0;

    send_transfers(seed, transfers, mwm).unwrap_or_else(|_| {
        *err = 1;
    })
}

fn send_transfers(seed: *const CSeed, transfers: *mut Transfers, mwm: u8) -> Result<()> {
    let seed = unsafe {
        assert!(!seed.is_null());
        &(*seed).0
    };

    let transfers = unsafe {
        assert!(!transfers.is_null());
       (*Box::from_raw(transfers)).0
    };

    let _ = smol::block_on(async move {
        iota::Client::send_transfers(&seed)
            .transfers(transfers)
            .min_weight_magnitude(mwm)
            .send()
            .await
    })?;

    // TODO retrun bundle

    Ok(())
}
