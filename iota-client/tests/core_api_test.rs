mod common;
use crate::common::*;

#[tokio::test]
async fn test_add_neighbors() {
    let client = client_init();
    let res = client
        .add_neighbors(&["tcp://0.0.0.0:15600"])
        .await
        .unwrap();

    //dbg!(res);
}
/*
#[tokio::test]
async fn test_attach_to_tangle() {
    let client = client_init();
    let res = client.attach_to_tangle(TEST_TRUNK_HASH, TEST_BRANCH_HASH, 9, &[TEST_TX_TRYTES]).await.unwrap();
    dbg!(res);
}
*/
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
/*
#[tokio::test]
async fn test_find_tx_by_empty_bundle() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        bundles: vec![NULL_HASH.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).await.unwrap();
    assert!(res.hashes().is_none());
}

#[tokio::test]
async fn test_find_tx_by_address() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        addresses: vec![TEST_ADDRESS_0.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).await.unwrap();
    assert!(res.hashes().is_some());
}

#[tokio::test]
async fn test_find_tx_by_empty_address() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        addresses: vec![NULL_HASH.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).await.unwrap();
    assert!(res.hashes().is_none());
}

#[tokio::test]
async fn test_find_tx_by_tag() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        tags: vec![TEST_TAG_0.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).await.unwrap();
    assert!(res.hashes().is_some());
}

#[tokio::test]
async fn test_find_tx_by_empty_tag() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        tags: vec![NULL_HASH.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).await.unwrap();
    assert!(res.hashes().is_none());
}

#[tokio::test]
async fn test_find_tx_by_approvee() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        approvees: vec![TEST_BUNDLE_TX_1.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).await.unwrap();
    assert!(res.hashes().is_some());
}

#[tokio::test]
async fn test_find_tx_by_empty_approvee() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        approvees: vec![NULL_HASH.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).await.unwrap();
    assert!(res.hashes().is_none());
}*/
/*
#[tokio::test]
async fn test_get_balances() {
    let client = client_init();
    let res = client.get_balances(&[TEST_ADDRESS_0], None, None).await.unwrap();
    dbg!(res);
    //assert_eq!(res.balances.len(), 1);
    //assert!(res.references.len() > 1);
}
*/
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

    assert!(res.hashes.len() > 0);
}
/*
#[tokio::test]
async fn test_get_transactions_to_approve() {
    let res = client_init()
        .get_transactions_to_approve(3, None)
        .await
        .unwrap();
}
*/
#[tokio::test]
async fn test_get_trytes() {
    let res = client_init()
        .get_trytes(&[TEST_BUNDLE_TX_1, TEST_BUNDLE_TX_0])
        .await
        .unwrap();

    assert_eq!(res.trytes.len(), 2);
}

#[tokio::test]
async fn test_remove_neighbors() {
    let res = client_init()
        .remove_neighbors(&["tcp://0.0.0.0:15600".into()])
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
