mod common;
use crate::common::*;
use iota_bundle_preview::*;
use iota_client::response::*;
use iota_client::Client;
use iota_crypto_preview::*;
use iota_signing_preview::*;
use iota_ternary_preview::*;

#[tokio::test]
async fn test_add_neighbors() {
    client_init();
    let _ = Client::add_neighbors(&["tcp://0.0.0.0:15600"])
        .await
        .unwrap();
}

#[tokio::test]
async fn test_attach_to_tangle() {
    client_init();
    let res = Client::attach_to_tangle()
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
}
/*
#[tokio::test]
async fn test_broadcast_bundle() {
    let _ = client_init()
        .broadcast_bundle(&Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}
*/

#[tokio::test]
async fn test_broadcast_transactions() {
    client_init();
    let _ = Client::broadcast_transactions(&[tx()]).await.unwrap();
}

#[tokio::test]
async fn test_check_consistency() {
    client_init();
    let res = Client::check_consistency(&[Hash::from_inner_unchecked(
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

#[tokio::test]
async fn test_find_tx_by_bundle() {
    client_init();
    let _ = Client::find_transactions()
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

#[tokio::test]
async fn test_find_tx_by_address() {
    client_init();
    let _ = Client::find_transactions()
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

#[tokio::test]
async fn test_find_tx_by_tag() {
    client_init();
    let _ = Client::find_transactions()
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

#[tokio::test]
async fn test_find_tx_by_approvee() {
    client_init();
    let _ = Client::find_transactions()
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

#[tokio::test]
async fn test_get_balances() {
    client_init();
    let _ = Client::get_balances()
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
/*
#[tokio::test]
async fn test_get_bundle() {
    let _ = client_init()
        .get_bundle(&Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}
*/

#[tokio::test]
async fn test_get_inclusion_states() {
    client_init();
    let res = Client::get_inclusion_states()
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
/*
#[tokio::test]
async fn test_get_inputs() {
    let _ = client_init()
        .get_inputs()
        .seed(
            &IotaSeed::<Kerl>::from_buf(
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

#[tokio::test]
async fn test_get_latest_inclusion() {
    let client = client_init();
    let res = client
        .get_latest_inclusion(&[
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

    assert!(!res.is_empty());
}

#[tokio::test]
async fn test_get_neighbors() {
    let client = client_init();

    match client.get_neighbors().await {
        Ok(res) => {
            assert!(res.neighbors.iter().all(|x| !x.address.is_empty()));
        }
        Err(e) => {
            let error = format!("{}", e);
            assert!(error.contains("COMMAND getNeighbors is not available on this node"));
        }
    }
}

#[tokio::test]
#[ignore]
async fn test_get_missing_transactions() {
    let _ = client_init().get_missing_transactions().await.unwrap();
}

#[tokio::test]
async fn test_get_new_address() {
    let _ = client_init()
        .get_new_address()
        .seed(
            &IotaSeed::<Kerl>::from_buf(
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

#[tokio::test]
#[ignore]
async fn test_get_node_api_configuration() {
    client_init().get_node_api_configuration().await.unwrap();
}

#[tokio::test]
async fn test_get_node_info() {
    client_init().get_node_info().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn test_get_tips() {
    let res = client_init().get_tips().await.unwrap();

    assert!(!res.hashes.is_empty());
}
*/

#[tokio::test]
async fn test_get_transactions_to_approve() {
    client_init();
    Client::get_transactions_to_approve()
        .depth(3)
        .send()
        .await
        .unwrap();
}
/*
#[tokio::test]
async fn test_get_trytes() {
    let res = client_init()
        .get_trytes()
        .hashes(&[
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

    assert!(!res.trytes.is_empty());
}

#[tokio::test]
async fn test_is_address_used() {
    let res = client_init()
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

#[tokio::test]
async fn test_is_promotable() {
    let _ = client_init()
        .is_promotable(&Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}

#[tokio::test]
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

    let _ = client_init()
        .prepare_transfers()
        .seed(
            &IotaSeed::<Kerl>::from_buf(
                TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                    .unwrap()
                    .as_trits()
                    .encode::<T1B1Buf>(),
            )
            .unwrap(),
        )
        .transfers(transfers)
        .build()
        .await
        .unwrap();
}
*/

#[tokio::test]
async fn test_remove_neighbors() {
    client_init();
    let res = Client::remove_neighbors(&["tcp://0.0.0.0:15600"])
        .await
        .unwrap();

    if let Some(neighbor) = res.removed_neighbors {
        assert_eq!(neighbor, 0);
    }
}
/*
#[tokio::test]
async fn test_replay_bundle() {
    let client = client_init();
    let _ = client
        .replay_bundle(&Hash::from_inner_unchecked(
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
// SRXEGEGCOZLDENAXVCGYODPUGTTRZJ9WCTYQYZIHWBCGNJRBBUOWGSDXQCQEHRIUEGA9G9IOEDCICZHOX
#[tokio::test]
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

    let _ = client_init()
        .send_transfers()
        .seed(
            &IotaSeed::<Kerl>::from_buf(
                TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                    .unwrap()
                    .as_trits()
                    .encode::<T1B1Buf>(),
            )
            .unwrap(),
        )
        .transfers(transfers)
        .min_weight_magnitude(10)
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_send_trytes() {
    let client = client_init();
    let _ = client
        .send_trytes()
        .min_weight_magnitude(9)
        .trytes(vec![tx()])
        .send()
        .await;
}
*/
#[tokio::test]
async fn test_store_and_broadcast() {
    client_init();
    Client::store_and_broadcast(&[tx()]).await.unwrap();
}

#[tokio::test]
async fn test_store_transactions() {
    client_init();
    Client::store_transactions(&[tx()]).await.unwrap();
}
/*
#[tokio::test]
async fn test_traverse_bundle() {
    let _ = client_init()
        .traverse_bundle(&Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}

#[tokio::test]
async fn test_were_addresses_spent_from() {
    let res = client_init()
        .were_addresses_spent_from()
        .address(&[Address::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_ADDRESS_0)
                .unwrap()
                .as_trits()
                .encode(),
        )])
        .send()
        .await
        .unwrap();

    assert_eq!(res.states[0], false);
}
*/
fn tx() -> Transaction {
    TransactionBuilder::new()
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
