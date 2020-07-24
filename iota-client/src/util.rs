use bee_transaction::bundled::{BundledTransaction as Transaction, BundledTransactionField};
use bee_transaction::TransactionVertex;
use bee_ternary::T3B1Buf;
use iota_conversion::{trytes, Trinary};

// TODO use bee-ternary once it porvides a method.
/// Temporary util function to make a transaction trytes
pub(crate) fn tx_trytes(tx: &Transaction) -> String {
    let bundle = tx.bundle().encode::<T3B1Buf>().iter_trytes().map(char::from).collect::<String>();
    trytes(tx.payload().to_inner().as_i8_slice()).unwrap()
        + &trytes(tx.address().to_inner().as_i8_slice()).unwrap()
        + &tx
            .value()
            .to_inner()
            .trits_with_length(81)
            .trytes()
            .unwrap()
        + &trytes(tx.obsolete_tag().to_inner().as_i8_slice()).unwrap()
        + &(*tx.timestamp().to_inner() as i64)
            .trits_with_length(27)
            .trytes()
            .unwrap()
        + &(*tx.index().to_inner() as i64)
            .trits_with_length(27)
            .trytes()
            .unwrap()
        + &(*tx.last_index().to_inner() as i64)
            .trits_with_length(27)
            .trytes()
            .unwrap()
        + &bundle 
        + &tx.trunk().encode::<T3B1Buf>().iter_trytes().map(char::from).collect::<String>()
        + &tx.branch().encode::<T3B1Buf>().iter_trytes().map(char::from).collect::<String>()
        + &trytes(tx.tag().to_inner().as_i8_slice()).unwrap()
        + &(*tx.attachment_ts().to_inner() as i64)
            .trits_with_length(27)
            .trytes()
            .unwrap()
        + &(*tx.attachment_lbts().to_inner() as i64)
            .trits_with_length(27)
            .trytes()
            .unwrap()
        + &(*tx.attachment_ubts().to_inner() as i64)
            .trits_with_length(27)
            .trytes()
            .unwrap()
        + &trytes(tx.nonce().to_inner().as_i8_slice()).unwrap()
}
