mod common;
use crate::common::*;
use bee_crypto::ternary::sponge::Kerl;
use bee_crypto::ternary::*;
use bee_signing::ternary::*;
use bee_ternary::*;
use bee_transaction::bundled::*;
use iota_client::response::*;
use iota_client::Url;

#[test]
fn test_attach_to_tangle() {
    smol::run(async {
        let client = client_init();
        let res = client
            .attach_to_tangle()
            .trunk_transaction(&Hash::from_inner_unchecked(
                TryteBuf::try_from_str(TEST_TRUNK_HASH)
                    .unwrap()
                    .as_trits()
                    .encode(),
            ))
            .branch_transaction(&Hash::from_inner_unchecked(
                TryteBuf::try_from_str(TEST_BRANCH_HASH)
                    .unwrap()
                    .as_trits()
                    .encode(),
            ))
            .min_weight_magnitude(10)
            .trytes(&[tx()])
            .send()
            .await
            .unwrap();
    
        assert!(!res.trytes.is_empty());
    })
}
/*
#[smol_potat::test]
async fn test_broadcast_bundle() {
    let client = client_init();
    let _ = client
        .broadcast_bundle(&Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_broadcast_transactions() {
    let client = client_init();
    let _ = client.broadcast_transactions(&[tx()]).await.unwrap();
}

#[smol_potat::test]
async fn test_check_consistency() {
    let client = client_init();
    let res = client
        .check_consistency(&[Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        )])
        .await
        .unwrap();

    match res.state {
        true => assert!(res.info.is_none()),
        false => assert!(res.info.is_some()),
    }
}

#[smol_potat::test]
async fn test_find_tx_by_bundle() {
    let client = client_init();
    let _ = client
        .find_transactions()
        .bundles(&[Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_HASH_0)
                .unwrap()
                .as_trits()
                .encode(),
        )])
        .send()
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_find_tx_by_address() {
    let client = client_init();
    let _ = client
        .find_transactions()
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
async fn test_find_tx_by_tag() {
    let client = client_init();
    let _ = client
        .find_transactions()
        .tags(&[Tag::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_TAG_0)
                .unwrap()
                .as_trits()
                .encode(),
        )])
        .send()
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_find_tx_by_approvee() {
    let client = client_init();
    let _ = client
        .find_transactions()
        .approvees(&[Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_1)
                .unwrap()
                .as_trits()
                .encode(),
        )])
        .send()
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_get_balances() {
    let client = client_init();
    let _ = client
        .get_balances()
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
async fn test_get_bundle() {
    let client = client_init();
    let _ = client
        .get_bundle(&Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_get_inclusion_states() {
    let client = client_init();
    let res = client
        .get_inclusion_states()
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
async fn test_get_inputs() {
    let client = client_init();
    let _ = client
        .get_inputs(
            &TernarySeed::<Kerl>::from_trits(
                TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                    .unwrap()
                    .as_trits()
                    .encode::<T1B1Buf>(),
            )
            .unwrap(),
        )
        .generate()
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_is_confirmed() {
    let client = client_init();
    let _ = client
        .is_confirmed(&[
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
async fn test_get_new_address() {
    let client = client_init();
    let _ = client
        .generate_new_address(
            &TernarySeed::<Kerl>::from_trits(
                TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                    .unwrap()
                    .as_trits()
                    .encode::<T1B1Buf>(),
            )
            .unwrap(),
        )
        .generate()
        .await
        .unwrap();
}

#[smol_potat::test]
#[ignore]
async fn test_get_node_api_configuration() {
    let client = client_init();
    client.get_node_api_configuration(Url::parse("https://nodes.comnet.thetangle.org").unwrap()).await.unwrap();
}

#[smol_potat::test]
async fn test_get_node_info() {
    let client = client_init();
    let _ = client.get_node_info(Url::parse("https://nodes.comnet.thetangle.org").unwrap()).await;
}

#[smol_potat::test]
async fn test_get_transactions_to_approve() {
    let client = client_init();
    client
        .get_transactions_to_approve()
        .depth(3)
        .send()
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_get_trytes() {
    let client = client_init();
    let res = client
        .get_trytes(&[
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
        .await
        .unwrap();

    assert!(!res.trytes.is_empty());
}

#[smol_potat::test]
async fn test_is_address_used() {
    let client = client_init();
    let res = client
        .is_address_used(&Address::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_ADDRESS_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();

    assert_eq!(res, false);
}

#[smol_potat::test]
async fn test_is_promotable() {
    let client = client_init();
    let _ = client
        .is_promotable(&Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_prepare_transfers_no_value() {
    let mut transfers = Vec::new();
    for _ in 0..3 {
        transfers.push(Transfer {
            address: Address::zeros(),
            value: 0,
            message: None,
            tag: None,
        });
    }

    let client = client_init();
    let _ = client
        .prepare_transfers(Some(
            &TernarySeed::<Kerl>::from_trits(
                TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                    .unwrap()
                    .as_trits()
                    .encode::<T1B1Buf>(),
            )
            .unwrap(),
        ))
        .transfers(transfers)
        .build()
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_reattach() {
    let client = client_init();
    let _ = client
        .reattach(&Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_HASH_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap()
        .depth(3)
        .min_weight_magnitude(9)
        .send()
        .await;
}

// We don't do value transfer test since it's not ideal to be a general test case. But confirmed sample can be found here:
#[smol_potat::test]
async fn test_send_transfers_no_value() {
    let mut transfers = Vec::new();
    for _ in 0..3 {
        transfers.push(Transfer {
            address: Address::from_inner_unchecked(
                TryteBuf::try_from_str(TEST_ADDRESS_0)
                    .unwrap()
                    .as_trits()
                    .encode(),
            ),
            value: 0,
            message: None,
            tag: None,
        });
    }

    let client = client_init();
    let _ = client
        .send(Some(
            &TernarySeed::<Kerl>::from_trits(
                TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                    .unwrap()
                    .as_trits()
                    .encode::<T1B1Buf>(),
            )
            .unwrap(),
        ))
        .transfers(transfers)
        .min_weight_magnitude(10)
        .send()
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_send_trytes() {
    let client = client_init();
    let _ = client
        .send_trytes()
        .min_weight_magnitude(9)
        .trytes(vec![tx()])
        .send()
        .await;
}

#[smol_potat::test]
async fn test_store_and_broadcast() {
    let client = client_init();
    client.store_and_broadcast(&[tx()]).await.unwrap();
}

#[smol_potat::test]
async fn test_store_transactions() {
    let client = client_init();
    client.store_transactions(&[tx()]).await.unwrap();
}

#[smol_potat::test]
async fn test_traverse_bundle() {
    let client = client_init();
    let _ = client
        .traverse_bundle(&Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}

#[smol_potat::test]
async fn test_were_addresses_spent_from() {
    let client = client_init();
    let res = client
        .were_addresses_spent_from(&[Address::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_ADDRESS_0)
                .unwrap()
                .as_trits()
                .encode(),
        )])
        .await
        .unwrap();

    assert_eq!(res.states[0], false);
}
*/
fn tx() -> BundledTransaction {
    BundledTransactionBuilder::new()
        .with_payload(Payload::zeros())
        .with_address(Address::zeros())
        .with_value(Value::from_inner_unchecked(0))
        .with_obsolete_tag(Tag::zeros())
        .with_timestamp(Timestamp::from_inner_unchecked(0))
        .with_index(Index::from_inner_unchecked(0))
        .with_last_index(Index::from_inner_unchecked(0))
        .with_tag(Tag::zeros())
        .with_attachment_ts(Timestamp::from_inner_unchecked(0))
        .with_bundle(Hash::zeros())
        .with_trunk(Hash::zeros())
        .with_branch(Hash::zeros())
        .with_attachment_lbts(Timestamp::from_inner_unchecked(0))
        .with_attachment_ubts(Timestamp::from_inner_unchecked(0))
        .with_nonce(Nonce::zeros())
        .build()
        .unwrap()
}
