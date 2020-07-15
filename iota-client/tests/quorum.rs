mod common;
use crate::common::*;
use bee_transaction::bundled::*;
use bee_crypto::ternary::Hash;
use iota_client::quorum;
use bee_ternary::*;

#[smol_potat::test]
async fn quorum_get_balances() {
    client_init();
    quorum::refresh_synced_nodes().await.unwrap();
    let _ = quorum::get_balances()
        .addresses(&[Address::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_ADDRESS_0)
                .unwrap()
                .as_trits()
                .encode(),
        )])
        .send()
        .await
        .unwrap();
}

#[smol_potat::test]
async fn quorum_get_inclusion_states() {
    client_init();
    quorum::refresh_synced_nodes().await.unwrap();
    let res = quorum::get_inclusion_states()
        .transactions(&[
            Hash::from_inner_unchecked(
                TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                    .unwrap()
                    .as_trits()
                    .encode(),
            ),
            Hash::from_inner_unchecked(
                TryteBuf::try_from_str(TEST_BUNDLE_TX_1)
                    .unwrap()
                    .as_trits()
                    .encode(),
            ),
        ])
        .send()
        .await
        .unwrap();

    assert!(!res.states.is_empty());
}

#[smol_potat::test]
async fn quorum_get_latest_inclusion() {
    client_init();
    quorum::refresh_synced_nodes().await.unwrap();
    let _ = quorum::get_latest_inclusion(&[
        Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ),
        Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_1)
                .unwrap()
                .as_trits()
                .encode(),
        ),
    ])
    .await;
}
#[smol_potat::test]
async fn quorum_were_addresses_spent_from() {
    client_init();
    quorum::refresh_synced_nodes().await.unwrap();
    let res = quorum::were_addresses_spent_from(&[Address::from_inner_unchecked(
        TryteBuf::try_from_str(TEST_ADDRESS_0)
            .unwrap()
            .as_trits()
            .encode(),
    )])
    .await
    .unwrap();

    assert_eq!(res.states[0], false);
}
