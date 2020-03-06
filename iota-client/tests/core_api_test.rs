mod common;
use crate::common::*;

#[tokio::test]
async fn test_add_neighbors() {
    let client = client_init();
    let _ = client
        .add_neighbors(&["tcp://0.0.0.0:15600"])
        .await
        .unwrap();
}

#[tokio::test]
async fn test_attach_to_tangle() {
    let client = client_init();
    let res = client
        .attach_to_tangle(TEST_TRUNK_HASH, TEST_BRANCH_HASH, 9, &[TEST_TX_TRYTES])
        .await
        .unwrap();
    assert!(!res.trytes.is_empty());
}

#[tokio::test]
async fn test_broadcast_transactions() {
    let client = client_init();
    let _ = client
        .broadcast_transactions(&[TEST_TX_TRYTES])
        .await
        .unwrap();
}

#[tokio::test]
async fn test_check_consistency() {
    let client = client_init();
    let res = client.check_consistency(&[TEST_BUNDLE_TX_0]).await.unwrap();

    match res.state {
        true => assert!(res.info.is_none()),
        false => assert!(res.info.is_some()),
    }
}

#[tokio::test]
async fn test_find_tx_by_bundle() {
    let client = client_init();
    let res = client
        .find_transactions(Some(&[TEST_BUNDLE_HASH_0]), None, None, None)
        .await
        .unwrap();

    assert!(!res.hashes.is_empty());
}

#[tokio::test]
async fn test_find_tx_by_address() {
    let client = client_init();
    let res = client
        .find_transactions(None, Some(&[TEST_ADDRESS_0]), None, None)
        .await
        .unwrap();

    assert!(!res.hashes.is_empty());
}

#[tokio::test]
async fn test_find_tx_by_tag() {
    let client = client_init();
    let res = client
        .find_transactions(None, None, Some(&[TEST_TAG_0]), None)
        .await
        .unwrap();

    assert!(!res.hashes.is_empty());
}

#[tokio::test]
async fn test_find_tx_by_approvee() {
    let client = client_init();
    let res = client
        .find_transactions(None, None, None, Some(&[TEST_BUNDLE_TX_1]))
        .await
        .unwrap();

    assert!(!res.hashes.is_empty());
}

#[tokio::test]
async fn test_get_balances() {
    let client = client_init();
    let _ = client
        .get_balances(&[TEST_ADDRESS_0], None, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_inclusion_states() {
    let client = client_init();
    let res = client
        .get_inclusion_states(&[TEST_BUNDLE_TX_0, TEST_BUNDLE_TX_1], None)
        .await
        .unwrap();

    assert!(res.states.iter().all(|x| *x));
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
async fn test_get_node_info() {
    client_init().get_node_info().await.unwrap();
}

#[tokio::test]
async fn test_get_tips() {
    let res = client_init().get_tips().await.unwrap();

    assert!(!res.hashes.is_empty());
}

#[tokio::test]
async fn test_get_transactions_to_approve() {
    let _ = client_init()
        .get_transactions_to_approve(3, None)
        .await
        .unwrap();
}

#[tokio::test]
async fn test_get_trytes() {
    let res = client_init()
        .get_trytes(&[TEST_BUNDLE_TX_1, TEST_BUNDLE_TX_0])
        .await
        .unwrap();

    assert!(!res.trytes.is_empty());
}

#[tokio::test]
async fn test_remove_neighbors() {
    let res = client_init()
        .remove_neighbors(&["tcp://0.0.0.0:15600"])
        .await
        .unwrap();

    if let Some(neighbor) = res.removed_neighbors {
        assert_eq!(neighbor, 0);
    }
}

#[tokio::test]
#[ignore]
async fn test_store_transactions() {
    client_init()
        .store_transactions(&[TEST_TX_TRYTES])
        .await
        .unwrap();
}

#[tokio::test]
async fn test_were_addresses_spent_from() {
    let res = client_init()
        .were_addresses_spent_from(&[TEST_ADDRESS_0])
        .await
        .unwrap();

    assert_eq!(res.states[0], true);
}
