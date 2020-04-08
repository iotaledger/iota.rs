mod common;
use crate::common::*;
use bee_bundle::*;
use bee_ternary::*;

#[tokio::test]
async fn test_add_neighbors() {
    let client = client_init();
    let _ = client
        .add_neighbors()
        .uris(&["tcp://0.0.0.0:15600"])
        .unwrap()
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_attach_to_tangle() {
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
        .min_weight_magnitude(9)
        .trytes(&[tx()])
        .send()
        .await
        .unwrap();

    assert!(!res.trytes.is_empty());
}

#[tokio::test]
async fn test_broadcast_bundle() {
    let _ = client_init()
        .broadcast_bundle(Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}

#[tokio::test]
async fn test_broadcast_transactions() {
    let client = client_init();
    let _ = client
        .broadcast_transactions()
        .trytes(&[tx()])
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_check_consistency() {
    let client = client_init();
    let res = client
        .check_consistency()
        .tails(&[Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        )])
        .send()
        .await
        .unwrap();

    match res.state {
        true => assert!(res.info.is_none()),
        false => assert!(res.info.is_some()),
    }
}

#[tokio::test]
async fn test_find_tx_by_bundle() {
    let client = client_init();
    let res = client
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

    assert!(!res.hashes.is_empty());
}

#[tokio::test]
async fn test_find_tx_by_address() {
    let client = client_init();
    let res = client
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

    assert!(!res.hashes.is_empty());
}

#[tokio::test]
async fn test_find_tx_by_tag() {
    let client = client_init();
    let res = client
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

    assert!(!res.hashes.is_empty());
}

#[tokio::test]
async fn test_find_tx_by_approvee() {
    let client = client_init();
    let res = client
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

    assert!(!res.hashes.is_empty());
}

#[tokio::test]
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

#[tokio::test]
async fn test_get_bundle() {
    let _ = client_init()
        .get_bundle(Hash::from_inner_unchecked(
            TryteBuf::try_from_str(TEST_BUNDLE_TX_0)
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .await
        .unwrap();
}

#[tokio::test]
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
async fn testget_missing_transactions() {
    let _ = client_init().get_missing_transactions().await.unwrap();
}

#[tokio::test]
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

#[tokio::test]
async fn test_get_transactions_to_approve() {
    let _ = client_init()
        .get_transactions_to_approve()
        .depth(3)
        .send()
        .await
        .unwrap();
}

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
async fn test_remove_neighbors() {
    let res = client_init()
        .remove_neighbors()
        .uris(&["tcp://0.0.0.0:15600"])
        .unwrap()
        .send()
        .await
        .unwrap();

    if let Some(neighbor) = res.removed_neighbors {
        assert_eq!(neighbor, 0);
    }
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

#[tokio::test]
async fn test_store_and_broadcast() {
    client_init().store_and_broadcast(&[tx()]).await.unwrap();
}

#[tokio::test]
async fn test_store_transactions() {
    client_init()
        .store_transactions()
        .trytes(&[tx()])
        .send()
        .await
        .unwrap();
}

#[tokio::test]
async fn test_traverse_bundle() {
    let _ = client_init()
        .traverse_bundle(Hash::from_inner_unchecked(
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

    assert_eq!(res.states[0], true);
}

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
