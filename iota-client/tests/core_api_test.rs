use iota_client::options::*;

mod common;
use crate::common::*;

#[test]
fn test_add_neighbors_empty() {
    let mut client = client_init();
    let res = client.add_neighbors(&vec!["".into()]).unwrap();

    if let Some(neighbor) = res.added_neighbors() {
        assert_eq!(*neighbor, 0);
    }
}

#[test]
fn test_attach_to_tangle_empty() {
    let mut client = client_init();

    let opt = AttachOptions::default();
    let res = client.attach_to_tangle(opt).unwrap_err();
    assert!(res
        .to_string()
        .contains("Provided trunk transaction is not valid"));
}

#[test]
fn test_attach_to_tangle_empty_trunk() {
    let mut client = client_init();

    let opt = AttachOptions {
        branch_transaction: TEST_BRANCH_HASH,
        trytes: &[TEST_TX_HASH.into()],
        ..AttachOptions::default()
    };
    let res = client.attach_to_tangle(opt).unwrap_err();
    assert!(res
        .to_string()
        .contains("Provided trunk transaction is not valid"));
}

#[test]
fn test_attach_to_tangle_empty_branch() {
    let mut client = client_init();

    let opt = AttachOptions {
        trunk_transaction: TEST_TRUNK_HASH,
        trytes: &[TEST_TX_HASH.into()],
        ..AttachOptions::default()
    };
    let res = client.attach_to_tangle(opt).unwrap_err();
    assert!(res
        .to_string()
        .contains("Provided branch transaction is not valid"));
}

#[test]
fn test_broadcast_transactions_empty() {
    let mut client = client_init();
    let res = client.broadcast_transactions(&["".into()]).unwrap_err();
    assert!(res.to_string().contains("Provided trytes are not valid"));
}

#[test]
fn test_check_consistency_empty() {
    let mut client = client_init();
    let res = client.check_consistency(&["".into()]).unwrap_err();
    assert!(res.to_string().contains("Provided hash is not valid"));
}

#[test]
fn test_check_consistency_not_tail() {
    let mut client = client_init();
    let res = client
        .check_consistency(&[TEST_BUNDLE_TX_1.into()])
        .unwrap();

    assert!(res["error"].is_string());
}

#[test]
fn test_check_consistency_empty_tail() {
    let mut client = client_init();
    let res = client.check_consistency(&[NULL_HASH.into()]).unwrap();

    assert!(res["error"].is_string());
}

#[test]
fn test_find_tx_empty() {
    let mut client = client_init();
    let opt = FindTransactionsOptions::default();
    let res = client.find_transactions(opt).unwrap();
    assert!(res.hashes().is_none());
}

#[test]
fn test_find_tx_by_bundle() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        bundles: vec![TEST_BUNDLE_TX_0.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).unwrap();
    assert!(res.hashes().is_some());
}

#[test]
fn test_find_tx_by_empty_bundle() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        bundles: vec![NULL_HASH.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).unwrap();
    assert!(res.hashes().is_none());
}

#[test]
fn test_find_tx_by_address() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        addresses: vec![TEST_ADDRESS_0.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).unwrap();
    assert!(res.hashes().is_some());
}

#[test]
fn test_find_tx_by_empty_address() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        addresses: vec![NULL_HASH.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).unwrap();
    assert!(res.hashes().is_none());
}

#[test]
fn test_find_tx_by_tag() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        tags: vec![TEST_TAG_0.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).unwrap();
    assert!(res.hashes().is_some());
}

#[test]
fn test_find_tx_by_empty_tag() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        tags: vec![NULL_HASH.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).unwrap();
    assert!(res.hashes().is_none());
}

#[test]
fn test_find_tx_by_approvee() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        approvees: vec![TEST_BUNDLE_TX_1.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).unwrap();
    assert!(res.hashes().is_some());
}

#[test]
fn test_find_tx_by_empty_approvee() {
    let mut client = client_init();
    let opt = FindTransactionsOptions {
        approvees: vec![NULL_HASH.into()],
        ..FindTransactionsOptions::default()
    };
    let res = client.find_transactions(opt).unwrap();
    assert!(res.hashes().is_none());
}

#[test]
fn test_get_balances() {
    let mut client = client_init();
    let opt = GetBalancesOptions {
        addresses: vec![TEST_ADDRESS_0.into()],
        ..GetBalancesOptions::default()
    };
    let res = client.get_balances(opt).unwrap();
    assert!(res.error().is_none());
}

#[test]
fn test_get_balances_empty() {
    let mut client = client_init();
    let opt = GetBalancesOptions {
        ..GetBalancesOptions::default()
    };
    let res = client.get_balances(opt).unwrap_err();
    assert!(res.to_string().contains("Provided addresses are not valid"));
}

#[test]
fn test_get_balances_with_tip() {
    let mut client = client_init();
    let opt = GetBalancesOptions {
        addresses: vec![TEST_ADDRESS_0.into()],
        tips: vec![TEST_BUNDLE_TX_0.into()],
        ..GetBalancesOptions::default()
    };
    let res = client.get_balances(opt).unwrap();
    assert!(res.error().is_none());
}

#[test]
fn test_get_balances_invalid_tip() {
    let mut client = client_init();
    let opt = GetBalancesOptions {
        addresses: vec![TEST_ADDRESS_0.into()],
        tips: vec![TEST_BUNDLE_HASH_0.into()],
        ..GetBalancesOptions::default()
    };
    let res = client.get_balances(opt).unwrap();
    assert!(res.error().is_some());
}

#[test]
fn test_get_inclusion_states() {
    let mut client = client_init();
    let opt = GetInclusionStatesOptions {
        transactions: vec![TEST_BUNDLE_TX_0.into()],
        tips: vec![TEST_MILESTONE_0.into()],
    };
    let res = client.get_inclusion_states(opt).unwrap();
    assert!(res.error().is_none());
}

#[test]
fn test_get_inclusion_states_empty() {
    let mut client = client_init();
    let opt = GetInclusionStatesOptions::default();
    let res = client.get_inclusion_states(opt).unwrap_err();
    assert!(res
        .to_string()
        .contains("Provided transactions are not valid"));
}

#[test]
fn test_get_inclusion_states_without_tip() {
    let mut client = client_init();
    let opt = GetInclusionStatesOptions {
        transactions: vec![TEST_BUNDLE_TX_0.into()],
        tips: vec![],
    };
    let res = client.get_inclusion_states(opt).unwrap();
    assert!(res.error().is_none());
}

#[test]
fn test_get_neighbors() {
    let mut client = client_init();

    match client.get_neighbors() {
        Ok(res) => {
            if let Some(neighbors) = res.clone().neighbors() {
                assert!(neighbors.iter().all(|x| !x.address.is_empty()));
            } else if let Some(error) = res.error() {
                assert!(!error.is_empty());
            } else {
                panic!("Invalid GetNeighbors Response.");
            }
        }
        Err(e) => {
            let error = format!("{}", e);
            assert!(error.contains("COMMAND getNeighbors is not available on this node"));
        }
    }
}

#[test]
fn test_get_node_info() {
    let mut client = client_init();
    let res = client.get_node_info().unwrap();
    println!("{:#?}", res);
    assert_ne!(res.app_name().len(), 0);
    assert_ne!(res.app_version().len(), 0);
    assert_ne!(res.latest_milestone(), NULL_HASH);
    assert!(res.latest_milestone_index() > STARTING_MILESTONE_INDEX);
    assert_ne!(res.latest_solid_subtangle_milestone(), NULL_HASH);
    assert!(res.latest_solid_subtangle_milestone_index() > STARTING_MILESTONE_INDEX);
    assert!(res.time() > OLDER_TIMESTAMP);
}

#[test]
fn test_get_tips() {
    let mut client = client_init();
    let res = client.get_tips().unwrap();

    assert!(!res.hashes().is_empty());
}

#[test]
fn test_get_transactions_to_approve() {
    let mut client = client_init();
    let opt = GetTransactionsToApproveOptions::default();
    let res = client.get_transactions_to_approve(opt).unwrap();
    assert!(res.trunk_transaction().is_some());
    assert!(res.branch_transaction().is_some());
}

#[test]
fn test_get_transactions_to_approve_invalid_depth() {
    let mut client = client_init();
    let opt = GetTransactionsToApproveOptions {
        depth: usize::max_value(),
        ..GetTransactionsToApproveOptions::default()
    };
    let res = client.get_transactions_to_approve(opt).unwrap_err();
    assert!(res.to_string().contains("Invalid depth input"));
}

#[test]
fn test_get_trytes() {
    let mut client = client_init();
    let res = client.get_trytes(&[TEST_BUNDLE_TX_1.into()]).unwrap();
    assert!(res.error().is_none());
}

#[test]
fn test_get_trytes_empty() {
    let mut client = client_init();
    let res = client.get_trytes(&["".into()]).unwrap_err();
    assert!(res.to_string().contains("Provided hashes are not valid"));
}

#[test]
fn test_remove_neighbors_empty() {
    let mut client = client_init();
    let res = client.remove_neighbors(&vec!["".into()]).unwrap();

    if let Some(neighbor) = res.removed_neighbors() {
        assert_eq!(*neighbor, 0);
    }
}

#[test]
fn test_store_transactions_empty() {
    let mut client = client_init();
    let res = client.store_transactions(&["".into()]).unwrap_err();
    assert!(res.to_string().contains("Provided trytes are not valid"));
}

#[test]
fn test_were_addresses_spent_from() {
    let mut client = client_init();
    let res = client
        .were_addresses_spent_from(&[TEST_ADDRESS_0.into()])
        .unwrap();
    assert!(res.error().is_none());
}
