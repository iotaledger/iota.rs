use bee_ternary::tryte::TryteBuf;
use bee_ternary::*;
use bee_transaction::bundled::{BundledTransaction as Transaction, BundledTransactionField};
use bee_transaction::TransactionVertex;

use std::convert::TryInto;

// TODO use bee-ternary once it porvides a method.
/// Temporary util function to make a transaction trytes
pub(crate) fn tx_trytes(tx: &Transaction) -> String {
    let bundle = tx
        .bundle()
        .encode::<T3B1Buf>()
        .iter_trytes()
        .map(char::from)
        .collect::<String>();

    fn num_to_tryte_string(num: i64, len: usize) -> String {
        let mut trytes: TritBuf<T1B1Buf> = num.into();
        let n = len - trytes.len();
        for _ in 0..n {
            trytes.push(Btrit::Zero);
        }
        trytes
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
    }

    tx.payload()
        .to_inner()
        .encode::<T3B1Buf>()
        .iter_trytes()
        .map(char::from)
        .collect::<String>()
        + &tx
            .address()
            .to_inner()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
        + &num_to_tryte_string(*tx.value().to_inner(), 81)
        + &tx
            .obsolete_tag()
            .to_inner()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
        + &num_to_tryte_string(*tx.timestamp().to_inner() as i64, 27)
        + &num_to_tryte_string(*tx.index().to_inner() as i64, 27)
        + &num_to_tryte_string(*tx.last_index().to_inner() as i64, 27)
        + &bundle
        + &tx
            .trunk()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
        + &tx
            .branch()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
        + &tx
            .tag()
            .to_inner()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
        + &num_to_tryte_string(*tx.attachment_ts().to_inner() as i64, 27)
        + &num_to_tryte_string(*tx.attachment_lbts().to_inner() as i64, 27)
        + &num_to_tryte_string(*tx.attachment_ubts().to_inner() as i64, 27)
        + &tx
            .nonce()
            .to_inner()
            .encode::<T3B1Buf>()
            .iter_trytes()
            .map(char::from)
            .collect::<String>()
}

/// Converts a UTF-8 string containing ascii into a tryte-encoded string
pub fn str_to_trytes(input: &str) -> TryteBuf {
    bytes_to_trytes(input.as_bytes())
}

/// Converts a sequence of bytes to trytes
pub fn bytes_to_trytes(input: &[u8]) -> TryteBuf {
    let mut trytes = TryteBuf::with_capacity(input.len() * 2);
    for byte in input {
        let first: i8 = match (byte % 27) as i8 {
            b @ 0..=13 => b,
            b @ 14..=26 => b - 27,
            _ => unreachable!(),
        };
        let second = match (byte / 27) as i8 {
            b @ 0..=13 => b,
            b @ 14..=26 => b - 27,
            _ => unreachable!(),
        };

        trytes.push(first.try_into().unwrap());
        trytes.push(second.try_into().unwrap());
    }
    trytes
}
