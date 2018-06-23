use failure::Error;
use reqwest::header::{ContentType, Headers};
use reqwest::{self, Response};
use serde_json::Value;
use super::utils::input_validator;

pub fn get_node_info(uri: &str) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getNodeInfo",
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn get_neighbors(uri: &str) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getNeighbors",
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn add_neighbors(uri: &str, uris: &[String]) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "addNeighbors",
        "uris": uris,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn remove_neighbors(uri: &str, uris: &[String]) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "removeNeighbors",
        "uris": uris,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn get_tips(uri: &str) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getTips",
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn find_transactions(
    uri: &str,
    bundles: Option<&[String]>,
    addresses: Option<&[String]>,
    tags: Option<&[String]>,
    approvees: Option<&[String]>,
) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let mut body = json!({
        "command": "findTransactions",
    });

    if let Some(b) = bundles {
        body["bundles"] = json!(b);
    }
    if let Some(a) = addresses {
        body["addresses"] = json!(a);
    }
    if let Some(t) = tags {
        body["tags"] = json!(t);
    }
    if let Some(a) = approvees {
        body["approvees"] = json!(a);
    }

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn get_trytes(uri: &str, hashes: &[String]) -> Result<Value, Error> {
    assert!(input_validator::is_array_of_hashes(hashes));

    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getTrytes",
        "hashes": hashes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn get_inclusion_states(
    uri: &str,
    transactions: &[String],
    tips: &[String],
) -> Result<Value, Error> {
    assert!(input_validator::is_array_of_hashes(transactions));
    assert!(input_validator::is_array_of_hashes(tips));

    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getInclusionStates",
        "transactions": transactions,
        "tips": tips,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn get_balances(uri: &str, addresses: &[String], threshold: i32) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getBalances",
        "addresses": addresses,
        "threshold": threshold,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn get_transactions_to_approve(uri: &str, depth: i32) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "getTransactionsToApprove",
        "depth": depth,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn attach_to_tangle(
    uri: &str,
    trunk_transaction: &str,
    branch_transaction: &str,
    min_weight_magnitude: i32,
    trytes: &[String],
) -> Result<Value, Error> {
    assert!(input_validator::is_hash(trunk_transaction));
    assert!(input_validator::is_hash(branch_transaction));
    assert!(input_validator::is_array_of_trytes(trytes));

    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "attachToTangle",
        "trunkTransaction": trunk_transaction,
        "branchTransaction": branch_transaction,
        "minWeightMagnitude": min_weight_magnitude,
        "trytes": trytes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn interrupt_attaching_to_tangle(uri: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    Ok(client.post(uri).send()?)
}

pub fn broadcast_transactions(uri: &str, trytes: &[String]) -> Result<Value, Error> {
    assert!(input_validator::is_array_of_attached_trytes(trytes));

    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "broadcastTransactions",
        "trytes": trytes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}

pub fn store_transactions(uri: &str, trytes: &[String]) -> Result<Value, Error> {
    let client = reqwest::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());
    headers.set_raw("X-IOTA-API-Version", "1");

    let body = json!({
        "command": "storeTransactions",
        "trytes": trytes,
    });

    Ok(client
        .post(uri)
        .headers(headers)
        .body(body.to_string())
        .send()?
        .json()?)
}
