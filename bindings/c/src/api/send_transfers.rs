use anyhow::Result;
use iota::crypto::Kerl;
use iota::signing::{IotaSeed, Seed};
use iota::ternary::Trits;
use std::slice;
use iota::client::Transfer;

#[repr(C)]
pub struct CTransfers {
    // TODO method to add transfers
    inner: Vec<Transfer>,
}

#[no_mangle]
pub extern "C" fn iota_send_transfers(seed: *const i8, transfers: *mut CTransfers, mwm: u8, err: &mut u8) {
    *err = 0;

    send_transfers(seed, transfers, mwm).unwrap_or_else(|_| {
        *err = 1;
    })
}

fn send_transfers(seed: *const i8, transfers: *mut CTransfers, mwm: u8) -> Result<()> {
    let seed = unsafe {
        assert!(!seed.is_null());

        slice::from_raw_parts(seed, 243)
    };
    let seed =
        IotaSeed::<Kerl>::from_buf(Trits::try_from_raw(seed, 243).unwrap().to_owned()).unwrap();

    let transfers = unsafe{ (*transfers).inner.clone() };
    let _ = smol::block_on(async move {
        iota::Client::send_transfers(&seed)
            .transfers(transfers)
            .min_weight_magnitude(mwm)
            .send()
            .await
    })?;

     Ok(())
}
